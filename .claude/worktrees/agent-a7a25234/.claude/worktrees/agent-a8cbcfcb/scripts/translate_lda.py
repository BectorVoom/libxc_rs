#!/usr/bin/env python3
"""
Translate maple2c LDA C source files to Rust #[cube] kernel functions.

Follows the exact pattern from src/kernel/lda/lda_x.rs.
"""

import re
import sys
import os
from pathlib import Path
from decimal import Decimal, getcontext

getcontext().prec = 50

# Map of C functions/macros to Rust equivalents
FUNC_MAP = {
    'POW_1_3': 'pow_1_3',
    'POW_2_3': 'pow_2_3',
    'POW_4_3': 'pow_4_3',
    'POW_5_3': 'pow_5_3',
    'POW_3_2': 'pow_3_2',
    'POW_1_4': 'pow_1_4',
    'POW_7_3': 'pow_7_3',
    'POW_2': 'pow_2',
    'my_piecewise3': 'piecewise3',
    'my_piecewise5': 'piecewise5',
    'sqrt': 'f64::sqrt',
    'log': 'f64::log',
    'exp': 'f64::exp',
    'fabs': 'f64::abs',
    'tanh': 'f64::tanh',
    'atan': 'f64::atan',
    'atan2': 'f64::atan2',
    'erf': 'erf_approx',
}

CONST_MAP = {
    'M_CBRT2': 'M_CBRT2', 'M_CBRT3': 'M_CBRT3', 'M_CBRT4': 'M_CBRT4',
    'M_CBRT5': 'M_CBRT5', 'M_CBRT6': 'M_CBRT6', 'M_CBRT7': 'M_CBRT7',
    'M_CBRT9': 'M_CBRT9', 'M_CBRTPI': 'M_CBRTPI',
    'M_SQRT2': 'M_SQRT2', 'M_SQRT3': 'M_SQRT3', 'M_SQRTPI': 'M_SQRTPI',
    'M_C': 'M_C',
}

OUTPUT_FIELDS = {
    'zk': ('zk', 1, 1),
    'vrho': ('vrho', 1, 2),
    'v2rho2': ('v2rho2', 1, 3),
    'v3rho3': ('v3rho3', 1, 4),
    'v4rho4': ('v4rho4', 1, 5),
}

ORDER_NAMES = ['exc', 'vxc', 'fxc', 'kxc', 'lxc']
ORDER_ARRAYS = {
    'exc': ['zk'],
    'vxc': ['zk', 'vrho'],
    'fxc': ['zk', 'vrho', 'v2rho2'],
    'kxc': ['zk', 'vrho', 'v2rho2', 'v3rho3'],
    'lxc': ['zk', 'vrho', 'v2rho2', 'v3rho3', 'v4rho4'],
}


def parse_c_functions(filepath):
    """Parse a maple2c C file into individual function bodies."""
    with open(filepath) as f:
        content = f.read()

    functions = {}
    # Find each function by searching for the signature and then extracting the body
    # by counting braces
    for m in re.finditer(r'func_(exc|vxc|fxc|kxc|lxc)_(unpol|pol)\s*\([^)]*\)\s*\{', content):
        order = m.group(1)
        spin = m.group(2)
        start = m.end()

        # Count braces to find matching close
        depth = 1
        pos = start
        while depth > 0 and pos < len(content):
            if content[pos] == '{':
                depth += 1
            elif content[pos] == '}':
                depth -= 1
            pos += 1

        body = content[start:pos-1]
        functions[(order, spin)] = body

    return functions


def extract_all_params(functions):
    """Extract all unique params references (excluding zeta_threshold, dens_threshold)."""
    all_params = set()
    for body in functions.values():
        for m in re.finditer(r'params->(\w+)(?:\[(\d+)\])?', body):
            field = m.group(1)
            if field in ('zeta_threshold', 'dens_threshold'):
                continue
            idx = int(m.group(2)) if m.group(2) is not None else None
            all_params.add((field, idx))
        # Also handle p->hyb_omega[N] and similar direct p-> references
        for m in re.finditer(r'p->(\w+)(?:\[(\d+)\])?', body):
            field = m.group(1)
            if field in ('zeta_threshold', 'dens_threshold', 'params', 'info', 'dim'):
                continue
            idx = int(m.group(2)) if m.group(2) is not None else None
            all_params.add((field, idx))
    return all_params


def param_to_arg_name(field, idx):
    if idx is not None:
        return f"param_{field}_{idx}"
    return f"param_{field}"


def translate_numeric_literal(match_str):
    """Convert a C numeric literal to Rust f64 literal with full precision."""
    try:
        val = float(match_str)
    except:
        return match_str

    # Use repr for full precision (Python repr gives shortest roundtrip-safe representation)
    r = repr(val)
    # Ensure it looks like a float
    if '.' not in r and 'e' not in r.lower() and r not in ('inf', '-inf', 'nan'):
        r = r + '.0'
    return r


def translate_line(line, is_pol):
    """Translate a single C statement to Rust, preserving structure."""
    line = line.strip()

    # Skip empty, preprocessor, braces
    if not line or line.startswith('#') or line in ('{', '}'):
        return None

    # Skip variable declarations
    if re.match(r'^double\s+', line):
        return None

    # Skip params struct lines
    if '_params *params' in line or re.match(r'^\s*params\s*=', line) or line.startswith('assert('):
        return None

    # Handle output writes: out->field[ip*p->dim.field + N] += varname;
    m = re.match(r'\s*if\s*\(\s*out->', line)
    if m:
        return None  # Skip the if-guard, we'll handle output writes directly

    m = re.match(r'\s*out->(\w+)\[ip\*p->dim\.\w+\s*\+\s*(\d+)\]\s*\+=\s*(\w+)\s*;', line)
    if m:
        field = m.group(1)
        component = int(m.group(2))
        varname = m.group(3)
        return ('output', field, component, varname)

    # Regular statement - strip semicolon
    if not line.endswith(';'):
        return None
    line = line[:-1].strip()

    # Translate the expression
    expr = translate_expr(line, is_pol)

    # Check if it's an assignment
    # Match: varname = expr (but not ==)
    m = re.match(r'^([a-zA-Z_]\w*)\s*=\s*(.+)$', expr)
    if m:
        varname = m.group(1)
        value = m.group(2)
        return ('let', varname, value)

    return ('stmt', expr)


def translate_expr(expr, is_pol):
    """Translate a C expression to Rust."""
    # Replace rho indexing
    if is_pol:
        expr = re.sub(r'rho\[0\]', 'rho0', expr)
        expr = re.sub(r'rho\[1\]', 'rho1', expr)
    else:
        expr = re.sub(r'rho\[0\]', 'rho[ip]', expr)

    # Replace M_PI
    expr = re.sub(r'\bM_PI\b', 'std::f64::consts::PI', expr)

    # Replace constants
    for c_const, r_const in CONST_MAP.items():
        expr = re.sub(r'\b' + c_const + r'\b', r_const, expr)

    # Replace params references
    def replace_param(m):
        field = m.group(1)
        idx = m.group(2)
        if idx is not None:
            return f"param_{field}_{idx}"
        return f"param_{field}"
    expr = re.sub(r'params->(\w+)(?:\[(\d+)\])?', replace_param, expr)

    # Replace p->zeta_threshold and dens_threshold
    expr = re.sub(r'p->zeta_threshold', 'zeta_threshold', expr)
    expr = re.sub(r'p->dens_threshold', 'dens_threshold', expr)

    # Replace p->field[idx] (e.g., p->hyb_omega[0]) with param_field_idx
    def replace_p_ref(m):
        field = m.group(1)
        idx = m.group(2)
        if field in ('params', 'info', 'dim'):
            return m.group(0)  # Don't touch these
        if idx is not None:
            return f"param_{field}_{idx}"
        return f"param_{field}"
    expr = re.sub(r'p->(\w+)(?:\[(\d+)\])?', replace_p_ref, expr)

    # Handle xc_integrate calls: replace with 0.0 stub (these need CPU-side pre-computation)
    expr = re.sub(r'xc_integrate\(func1,\s*NULL,\s*[^,]+,\s*([^)]+)\)', r'0.0 /* TODO: xc_integrate(func1, \1) */', expr)
    expr = re.sub(r'xc_integrate\(func2,\s*NULL,\s*[^,]+,\s*([^)]+)\)', r'0.0 /* TODO: xc_integrate(func2, \1) */', expr)

    # Replace C function calls with Rust equivalents
    for c_func, r_func in FUNC_MAP.items():
        expr = re.sub(r'\b' + re.escape(c_func) + r'\b', r_func, expr)

    # Replace pow(x, y) with f64::powf(x, y)
    expr = re.sub(r'\bpow\b(?!_)', 'f64::powf', expr)

    # Translate numeric literals (careful ordering)
    # Match float literals: digits.digits[eExponent]
    def replace_num(m):
        return translate_numeric_literal(m.group(0))
    expr = re.sub(r'(?<![a-zA-Z_\d])(\d+\.\d+(?:[eE][+-]?\d+)?)', replace_num, expr)

    # Also handle integer-like literals that are clearly meant as floats in context
    # e.g., standalone "1" in piecewise3(..., 1) should be 1.0
    # But this is tricky - leave as-is for now, since maple2c usually uses 0.1e1 style

    # Fix standalone integer constants in piecewise calls
    # In C: my_piecewise3(cond, val_true, 1) -> piecewise3(cond, val_true, 1.0)
    expr = re.sub(r'(piecewise3\([^)]*,\s*)(\d+)\)', lambda m: m.group(1) + m.group(2) + '.0)', expr)
    # For piecewise5 the last 3 args may be integers
    expr = re.sub(r'(piecewise5\([^)]*,\s*)(\d+)\)', lambda m: m.group(1) + m.group(2) + '.0)', expr)

    return expr


def compute_output_index(field, component, is_pol):
    """Compute array index expression for output write."""
    if not is_pol:
        return "ip"
    else:
        _, _, n_pol = OUTPUT_FIELDS[field]
        if n_pol == 1:
            return "ip"
        if component == 0:
            return f"ip * {n_pol}"
        return f"ip * {n_pol} + {component}"


def generate_rust_function(func_name, order, spin, body, all_params):
    """Generate a Rust #[cube] function from C function body."""
    is_pol = (spin == 'pol')
    arrays = ORDER_ARRAYS[order]
    fn_name = f"{func_name}_{order}_{spin}"
    spin_label = 'polarized' if is_pol else 'unpolarized'

    sorted_params = sorted(all_params, key=lambda x: (x[0], x[1] if x[1] is not None else -1))

    # Build signature
    sig_parts = []
    sig_parts.append(f"    rho: &Array<f64>,")
    for arr in arrays:
        sig_parts.append(f"    {arr}: &mut Array<f64>,")
    for field, idx in sorted_params:
        arg_name = param_to_arg_name(field, idx)
        sig_parts.append(f"    {arg_name}: f64,")
    sig_parts.append(f"    dens_threshold: f64,")
    sig_parts.append(f"    #[allow(unused_variables)] zeta_threshold: f64,")

    # Parse and translate lines
    translated = []
    for raw_line in body.split('\n'):
        result = translate_line(raw_line, is_pol)
        if result is None:
            continue
        translated.append(result)

    # Build function body
    body_lines = []
    if is_pol:
        body_lines.append("        let rho0 = rho[ip * 2];")
        body_lines.append("        let rho1 = rho[ip * 2 + 1];")
        body_lines.append("")

    for item in translated:
        if item[0] == 'let':
            _, varname, value = item
            body_lines.append(f"        let {varname} = {value};")
        elif item[0] == 'output':
            _, field, component, varname = item
            idx = compute_output_index(field, component, is_pol)
            body_lines.append(f"        {field}[{idx}] += {varname};")
        elif item[0] == 'stmt':
            body_lines.append(f"        {item[1]};")

    # Assemble
    lines = []
    lines.append(f"/// {func_name.upper()} {order} -- {spin_label}.")
    lines.append(f"#[cube(launch_unchecked)]")
    lines.append(f"pub fn {fn_name}(")
    lines.extend(sig_parts)
    lines.append(f") {{")
    lines.append(f"    let ip = ABSOLUTE_POS;")
    lines.append(f"    if ip < zk.len() {{")
    lines.extend(body_lines)
    lines.append(f"    }}")
    lines.append(f"}}")

    return '\n'.join(lines)


def determine_imports(all_params, functions):
    """Determine needed imports."""
    all_text = '\n'.join(functions.values())
    imports = ["use cubecl::prelude::*;"]

    # Constants - use word boundary regex
    used_consts = []
    for c_const, r_const in CONST_MAP.items():
        if re.search(r'\b' + c_const + r'\b', all_text):
            used_consts.append(r_const)
    if used_consts:
        imports.append(f"use crate::math::constants::{{{', '.join(sorted(set(used_consts)))}}};")

    # Power functions
    power_map = {
        'POW_1_3': 'pow_1_3', 'POW_2_3': 'pow_2_3', 'POW_4_3': 'pow_4_3',
        'POW_5_3': 'pow_5_3', 'POW_3_2': 'pow_3_2', 'POW_1_4': 'pow_1_4',
        'POW_7_3': 'pow_7_3', 'POW_2': 'pow_2',
    }
    used_powers = [v for k, v in power_map.items() if k in all_text]
    if used_powers:
        imports.append(f"use crate::math::powers::{{{', '.join(sorted(set(used_powers)))}}};")

    if 'my_piecewise3' in all_text and 'my_piecewise5' in all_text:
        imports.append("use crate::math::piecewise::{piecewise3, piecewise5};")
    elif 'my_piecewise3' in all_text:
        imports.append("use crate::math::piecewise::piecewise3;")
    elif 'my_piecewise5' in all_text:
        imports.append("use crate::math::piecewise::piecewise5;")

    if re.search(r'\berf\b', all_text) and 'erfc' not in all_text:
        imports.append("use crate::math::erf::erf_approx;")
    elif 'erfc' in all_text:
        imports.append("use crate::math::erf::{erf_approx, erfc_approx};")

    return imports


def generate_kernel_file(func_name, c_filepath, vxc_filepath=None):
    """Generate complete Rust kernel file."""
    functions = parse_c_functions(c_filepath)
    if vxc_filepath and os.path.exists(vxc_filepath):
        vxc_functions = parse_c_functions(vxc_filepath)
        functions.update(vxc_functions)

    all_params = extract_all_params(functions)
    imports = determine_imports(all_params, functions)

    available_orders = []
    for order in ORDER_NAMES:
        if (order, 'unpol') in functions or (order, 'pol') in functions:
            available_orders.append(order)

    lines = []
    lines.append(f"//! {func_name.upper()} kernel functions translated from libxc maple2c.")
    lines.append(f"//!")
    lines.append(f"//! Auto-translated. Preserves exact maple2c variable names and operation order.")
    lines.append(f"#![allow(clippy::excessive_precision, clippy::needless_return, unused_variables)]")
    lines.append(f"")
    for imp in imports:
        lines.append(imp)
    lines.append(f"")

    lines.append("// ============================================================================")
    lines.append("// UNPOLARIZED FUNCTIONS")
    lines.append("// ============================================================================")
    lines.append("")

    for order in available_orders:
        key = (order, 'unpol')
        if key in functions:
            fn_code = generate_rust_function(func_name, order, 'unpol', functions[key], all_params)
            lines.append(fn_code)
            lines.append("")

    lines.append("// ============================================================================")
    lines.append("// POLARIZED FUNCTIONS")
    lines.append("// ============================================================================")
    lines.append("")

    for order in available_orders:
        key = (order, 'pol')
        if key in functions:
            fn_code = generate_rust_function(func_name, order, 'pol', functions[key], all_params)
            lines.append(fn_code)
            lines.append("")

    return '\n'.join(lines), all_params, available_orders, functions


def generate_launch_file(func_name, all_params, available_orders, functions):
    """Generate launch wrapper file."""
    sorted_params = sorted(all_params, key=lambda x: (x[0], x[1] if x[1] is not None else -1))

    lines = []
    lines.append(f"//! Safe launch wrappers for {func_name.upper()} CubeCL kernels.")
    lines.append(f"#![allow(unused_variables)]")
    lines.append(f"")
    lines.append(f"use cubecl::cpu::CpuRuntime;")
    lines.append(f"use cubecl::client::ComputeClient;")
    lines.append(f"use cubecl::prelude::*;")
    lines.append(f"")
    lines.append(f"use super::{func_name};")
    lines.append(f"use super::launch_lda_x::BufArg;")
    lines.append(f"")

    for order in available_orders:
        for spin in ['unpol', 'pol']:
            key = (order, spin)
            if key not in functions:
                continue

            arrays = ORDER_ARRAYS[order]
            fn_name = f"launch_{func_name}_{order}_{spin}"
            kernel_fn = f"{func_name}::{func_name}_{order}_{spin}"

            sig = []
            sig.append(f"    client: &ComputeClient<CpuRuntime>,")
            sig.append(f"    cube_count: CubeCount,")
            sig.append(f"    cube_dim: CubeDim,")
            sig.append(f"    rho: &BufArg<'_>,")
            for arr in arrays:
                sig.append(f"    {arr}: &BufArg<'_>,")
            for field, idx in sorted_params:
                sig.append(f"    {param_to_arg_name(field, idx)}: f64,")
            sig.append(f"    dens_threshold: f64,")
            sig.append(f"    zeta_threshold: f64,")

            launch_args = []
            launch_args.append(f"            ArrayArg::from_raw_parts::<f64>(rho.handle, rho.len, 1),")
            for arr in arrays:
                launch_args.append(f"            ArrayArg::from_raw_parts::<f64>({arr}.handle, {arr}.len, 1),")
            for field, idx in sorted_params:
                launch_args.append(f"            ScalarArg::new({param_to_arg_name(field, idx)}),")
            launch_args.append(f"            ScalarArg::new(dens_threshold),")
            launch_args.append(f"            ScalarArg::new(zeta_threshold),")

            lines.append(f"pub fn {fn_name}(")
            lines.extend(sig)
            lines.append(f") -> Result<(), Box<dyn std::error::Error>> {{")
            lines.append(f"    unsafe {{")
            lines.append(f"        {kernel_fn}::launch_unchecked::<CpuRuntime>(")
            lines.append(f"            client, cube_count, cube_dim,")
            lines.extend(launch_args)
            lines.append(f"        )?;")
            lines.append(f"    }}")
            lines.append(f"    Ok(())")
            lines.append(f"}}")
            lines.append(f"")

    return '\n'.join(lines)


def main():
    c_dir = Path(sys.argv[1])
    rust_dir = Path(sys.argv[2])
    specific = sys.argv[3] if len(sys.argv) > 3 else None

    functionals = [
        'lda_c_1d_csc', 'lda_c_1d_loos', 'lda_c_2d_amgb', 'lda_c_2d_prm',
        'lda_c_chachiyo', 'lda_c_chachiyo_mod', 'lda_c_gk72', 'lda_c_gombas',
        'lda_c_hl', 'lda_c_lp96', 'lda_c_ml1', 'lda_c_pk09', 'lda_c_pmgb06',
        'lda_c_pw', 'lda_c_pw_erf', 'lda_c_pz', 'lda_c_rc04', 'lda_c_rpa',
        'lda_c_vwn', 'lda_c_vwn_1', 'lda_c_vwn_2', 'lda_c_vwn_3', 'lda_c_vwn_4',
        'lda_c_vwn_rpa', 'lda_c_w20', 'lda_c_wigner',
        'lda_k_gds08_worker', 'lda_k_tf', 'lda_k_zlp',
        'lda_x_1d_exponential', 'lda_x_1d_soft', 'lda_x_2d', 'lda_x_erf',
        'lda_x_rel', 'lda_x_sloc', 'lda_x_yukawa',
        'lda_xc_1d_ehwlrg', 'lda_xc_ksdt', 'lda_xc_teter93', 'lda_xc_zlp',
        'hyb_lda_xc_bn05',
    ]

    if specific:
        functionals = [specific]

    rust_dir.mkdir(parents=True, exist_ok=True)

    for func_name in functionals:
        c_file = c_dir / f"{func_name}.c"
        if not c_file.exists():
            print(f"WARNING: {c_file} not found, skipping")
            continue

        print(f"Translating {func_name}...")
        kernel_code, all_params, available_orders, functions = generate_kernel_file(func_name, str(c_file))

        kernel_path = rust_dir / f"{func_name}.rs"
        with open(kernel_path, 'w') as f:
            f.write(kernel_code)

        launch_code = generate_launch_file(func_name, all_params, available_orders, functions)
        launch_path = rust_dir / f"launch_{func_name}.rs"
        with open(launch_path, 'w') as f:
            f.write(launch_code)

        print(f"  -> {kernel_path} ({len(available_orders)} orders)")

    # Handle special lda_xc_tih from lda_vxc/
    tih_file = c_dir.parent / 'lda_vxc' / 'lda_xc_tih.c'
    if tih_file.exists() and (specific is None or specific == 'lda_xc_tih'):
        print(f"Translating lda_xc_tih (special _vxc)...")
        kernel_code, all_params, available_orders, functions = generate_kernel_file(
            'lda_xc_tih', str(tih_file)
        )
        kernel_path = rust_dir / 'lda_xc_tih.rs'
        with open(kernel_path, 'w') as f:
            f.write(kernel_code)
        launch_code = generate_launch_file('lda_xc_tih', all_params, available_orders, functions)
        launch_path = rust_dir / 'launch_lda_xc_tih.rs'
        with open(launch_path, 'w') as f:
            f.write(launch_code)
        print(f"  -> {kernel_path} ({len(available_orders)} orders)")


if __name__ == '__main__':
    main()
