#!/usr/bin/env python3
"""Translate maple2c C kernel files to Rust #[cube] functions.

Reads auto-generated maple2c C files and produces Rust source files
following the exact pattern established in src/kernel/lda/lda_x.rs.
"""

import re
import sys
import os
from pathlib import Path


def parse_c_functions(c_code: str) -> list:
    """Parse all func_*_unpol and func_*_pol functions from C source."""
    functions = []

    func_pattern = re.compile(
        r'func_(exc|vxc|fxc|kxc|lxc)_(unpol|pol)\s*\([^)]*\)\s*\{',
        re.MULTILINE
    )

    for match in func_pattern.finditer(c_code):
        level = match.group(1)
        spin = match.group(2)
        start = match.end()

        depth = 1
        pos = start
        while depth > 0 and pos < len(c_code):
            if c_code[pos] == '{':
                depth += 1
            elif c_code[pos] == '}':
                depth -= 1
            pos += 1

        body = c_code[start:pos-1]
        functions.append({
            'level': level,
            'spin': spin,
            'body': body,
        })

    return functions


def extract_params(c_code: str) -> list:
    """Extract unique parameter names used (params->xxx patterns)."""
    seen = []
    seen_set = set()
    for match in re.finditer(r'params->(\w+)(?:\[(\d+)\])?', c_code):
        name = match.group(1).lower()
        idx = match.group(2)
        key = f"{name}_{idx}" if idx is not None else name
        if key not in seen_set:
            seen.append(key)
            seen_set.add(key)
    # Also check p->field[idx] patterns (e.g., p->hyb_omega[0])
    for match in re.finditer(r'p->(\w+)\[(\d+)\]', c_code):
        name = match.group(1).lower()
        idx = match.group(2)
        if name in ('zeta_threshold', 'dens_threshold', 'info'):
            continue
        key = f"{name}_{idx}"
        if key not in seen_set:
            seen.append(key)
            seen_set.add(key)
    return seen


def detect_math_functions(c_code: str) -> set:
    funcs = set()
    for name in ['POW_1_3', 'POW_2_3', 'POW_4_3', 'POW_5_3', 'POW_3_2', 'POW_1_4', 'POW_7_3']:
        if name in c_code:
            funcs.add(name.lower())
    if 'POW_2(' in c_code:
        funcs.add('pow_2')
    if 'my_piecewise3' in c_code:
        funcs.add('piecewise3')
    if 'my_piecewise5' in c_code:
        funcs.add('piecewise5')
    return funcs


def detect_constants(c_code: str) -> set:
    consts = set()
    for name in ['M_CBRT2', 'M_CBRT3', 'M_CBRT4', 'M_CBRT5', 'M_CBRT6', 'M_CBRT7',
                 'M_CBRT9', 'M_CBRTPI', 'M_SQRTPI', 'M_SQRT2', 'M_SQRT3', 'M_C']:
        # Use word boundary to avoid false matches (e.g. M_CBRT3 matching M_C)
        if re.search(r'\b' + name + r'\b', c_code):
            consts.add(name)
    return consts


def translate_numeric(text: str) -> str:
    """Translate a single maple2c numeric literal."""
    m = re.match(r'^([-]?)(\d+\.\d+)e([+-]?\d+)$', text, re.IGNORECASE)
    if m:
        sign = m.group(1)
        mantissa = float(m.group(2))
        exponent = int(m.group(3))
        value = mantissa * (10 ** exponent)
        if value == int(value) and abs(value) < 1e15:
            result = f"{sign}{int(value)}.0"
        else:
            result = f"{sign}{value:.15g}"
            if '.' not in result and 'e' not in result.lower():
                result += '.0'
        return result
    return text


def translate_expr(expr: str, spin: str) -> str:
    """Translate a C expression to Rust."""
    # POW macros
    for name in ['POW_1_3', 'POW_2_3', 'POW_4_3', 'POW_5_3', 'POW_3_2', 'POW_1_4', 'POW_7_3']:
        expr = expr.replace(f'{name}(', f'{name.lower()}(')
    expr = re.sub(r'POW_2\(', 'pow_2(', expr)

    # Piecewise
    expr = expr.replace('my_piecewise3(', 'piecewise3(')
    expr = expr.replace('my_piecewise5(', 'piecewise5(')

    # Math functions -> CubeCL f64 methods
    expr = re.sub(r'\bsqrt\(', 'f64::sqrt(', expr)
    expr = re.sub(r'\blog\(', 'f64::ln(', expr)
    expr = re.sub(r'\bexp\(', 'f64::exp(', expr)
    expr = re.sub(r'\batan\(', 'f64::atan(', expr)
    expr = re.sub(r'\btanh\(', 'f64::tanh(', expr)
    expr = re.sub(r'\bpow\(([^,]+),\s*([^)]+)\)', r'f64::powf(\1, \2)', expr)
    expr = re.sub(r'\batan2\(([^,]+),\s*([^)]+)\)', r'f64::atan2(\1, \2)', expr)
    expr = re.sub(r'\bfabs\(([^)]+)\)', r'f64::abs(\1)', expr)
    # erf/erfc -> our approximation functions
    expr = re.sub(r'\berfc\(', 'erfc_approx(', expr)
    expr = re.sub(r'\berf\(', 'erf_approx(', expr)

    # M_PI
    expr = re.sub(r'\bM_PI\b', 'M_PI_VAL', expr)

    # rho indexing
    if spin == 'unpol':
        expr = re.sub(r'rho\[0\]', 'rho[ip]', expr)
    else:
        expr = re.sub(r'rho\[0\]', 'rho[ip * 2]', expr)
        expr = re.sub(r'rho\[1\]', 'rho[ip * 2 + 1]', expr)

    # zeta/dens threshold
    expr = re.sub(r'p->zeta_threshold', 'zeta_threshold', expr)
    expr = re.sub(r'p->dens_threshold', 'dens_threshold', expr)

    # p->hyb_omega[idx] -> param_hyb_omega_idx
    def p_sub(m):
        name = m.group(1).lower()
        idx = m.group(2)
        return f'param_{name}_{idx}'
    expr = re.sub(r'p->(\w+)\[(\d+)\]', p_sub, expr)

    # params->field[idx] and params->field -- lowercase for Rust snake_case
    def param_sub_idx(m):
        name = m.group(1).lower()
        idx = m.group(2)
        return f'param_{name}_{idx}'
    def param_sub(m):
        name = m.group(1).lower()
        return f'param_{name}'
    expr = re.sub(r'params->(\w+)\[(\d+)\]', param_sub_idx, expr)
    expr = re.sub(r'params->(\w+)', param_sub, expr)

    # Numeric literals
    expr = re.sub(r'[-]?\d+\.\d+e[+-]?\d+', lambda m: translate_numeric(m.group(0)), expr)

    # Integer literals used as f64 -> add .0 suffix
    # Match bare integers that are function arguments or in arithmetic
    # Pattern: comma/space followed by a bare integer followed by comma/paren/space/operator
    # Be careful not to match array indices like [ip * 2 + 1]
    # Strategy: replace bare integers in comma-separated contexts (function args)
    def int_to_float(m):
        pre = m.group(1)
        num = m.group(2)
        post = m.group(3)
        return f'{pre}{num}.0{post}'

    # Match: , N) or , N, (function arguments)
    expr = re.sub(r'(,\s*)(\d+)(\s*[,)])', int_to_float, expr)
    # Match: (N, at start of function arg list
    expr = re.sub(r'(\(\s*)(\d+)(\s*,)', int_to_float, expr)

    return expr


def get_output_fields(level: str, is_vxc_only: bool = False) -> list:
    fields = []
    if not is_vxc_only:
        fields.append('zk')
    if level in ('vxc', 'fxc', 'kxc', 'lxc'):
        fields.append('vrho')
    elif not is_vxc_only:
        fields.append('zk')  # exc only has zk
        return ['zk']
    if level in ('fxc', 'kxc', 'lxc'):
        fields.append('v2rho2')
    if level in ('kxc', 'lxc'):
        fields.append('v3rho3')
    if level == 'lxc':
        fields.append('v4rho4')
    return fields


def process_body(body: str, level: str, spin: str) -> tuple:
    """Process C function body into (code_lines, output_assignments)."""
    lines = body.strip().split('\n')
    code_lines = []
    outputs = []

    for line in lines:
        s = line.strip()
        if not s:
            continue
        # Skip declarations
        if s.startswith('double '):
            continue
        if re.match(r'\w+_params\s+\*', s):
            continue
        if 'assert(' in s:
            continue
        if re.match(r'params\s*=\s*\(', s):
            continue

        # Parse output assignment: out->field[ip*p->dim.field + idx] += var;
        out_m = re.match(
            r'out->(zk|vrho|v2rho2|v3rho3|v4rho4)\[ip\s*\*\s*p->dim\.\w+\s*\+\s*(\d+)\]\s*\+=\s*(\w+)\s*;',
            s
        )
        if out_m:
            field = out_m.group(1)
            idx = int(out_m.group(2))
            var = out_m.group(3)
            outputs.append((field, var, idx))
            continue

        # Skip null checks
        if s.startswith('if(out->'):
            continue

        # Remove trailing semicolon
        if s.endswith(';'):
            s = s[:-1]

        translated = translate_expr(s, spin)

        # Make assignment into let binding
        if '=' in translated and not translated.startswith('//'):
            parts = translated.split('=', 1)
            if re.match(r'^\s*\w+\s*$', parts[0]):
                translated = 'let ' + translated

        code_lines.append(translated + ';')

    return code_lines, outputs


def gen_function(func_name: str, level: str, spin: str, body: str,
                 has_params: bool, param_names: list,
                 is_vxc_only: bool = False) -> str:
    """Generate a complete Rust #[cube] function."""
    fields = get_output_fields(level, is_vxc_only)

    code_lines, outputs = process_body(body, level, spin)

    # Build signature
    sig_parts = ['    rho: &Array<f64>']
    for f in fields:
        sig_parts.append(f'    {f}: &mut Array<f64>')
    if has_params:
        for p in param_names:
            sig_parts.append(f'    param_{p}: f64')
    sig_parts.append('    #[allow(unused_variables)] dens_threshold: f64')
    sig_parts.append('    #[allow(unused_variables)] zeta_threshold: f64')
    sig = ',\n'.join(sig_parts)

    bounds_field = fields[0] if fields else 'zk'

    # Build body
    b = []
    b.append('    let ip = ABSOLUTE_POS;')
    b.append(f'    if ip < {bounds_field}.len() {{')

    for cl in code_lines:
        b.append(f'        {cl}')

    # Accumulate outputs
    for (field, var, idx) in outputs:
        if spin == 'unpol':
            b.append(f'        {field}[ip] += {var};')
        else:
            dim = {'zk': 1, 'vrho': 2, 'v2rho2': 3, 'v3rho3': 4, 'v4rho4': 5}[field]
            if dim == 1:
                b.append(f'        {field}[ip] += {var};')
            else:
                b.append(f'        {field}[ip * {dim} + {idx}] += {var};')

    b.append('    }')

    body_str = '\n'.join(b)

    return f"""#[allow(unused_variables)]
#[cube(launch_unchecked)]
pub fn {func_name}_{level}_{spin}(
{sig},
) {{
{body_str}
}}"""


def translate_file(c_path: str, func_name: str, is_vxc_only: bool = False) -> str:
    """Translate an entire C file to Rust."""
    with open(c_path) as f:
        c_code = f.read()

    math_funcs = detect_math_functions(c_code)
    constants = detect_constants(c_code)
    has_params = 'params->' in c_code or bool(re.search(r'p->\w+\[\d+\]', c_code))
    param_names = extract_params(c_code)
    has_pi = bool(re.search(r'\bM_PI\b', c_code))

    # Build imports
    imports = ['use cubecl::prelude::*;']

    if constants:
        imports.append(f'use crate::math::constants::{{{", ".join(sorted(constants))}}};')

    piecewise = sorted([f for f in math_funcs if f.startswith('piecewise')])
    powers = sorted([f for f in math_funcs if f.startswith('pow_')])

    # Check for erf/erfc usage
    has_erf = bool(re.search(r'\berf\(', c_code))
    has_erfc = bool(re.search(r'\berfc\(', c_code))
    erf_imports = []
    if has_erf:
        erf_imports.append('erf_approx')
    if has_erfc:
        erf_imports.append('erfc_approx')

    if piecewise:
        imports.append(f'use crate::math::piecewise::{{{", ".join(piecewise)}}};')
    if powers:
        imports.append(f'use crate::math::powers::{{{", ".join(powers)}}};')
    if erf_imports:
        imports.append(f'use crate::math::erf::{{{", ".join(sorted(erf_imports))}}};')

    pi_const = ""
    if has_pi:
        pi_const = "\nconst M_PI_VAL: f64 = std::f64::consts::PI;\n"

    # Parse and translate functions
    functions = parse_c_functions(c_code)

    unpol = [f for f in functions if f['spin'] == 'unpol']
    pol = [f for f in functions if f['spin'] == 'pol']

    parts = []

    if unpol:
        parts.append("// ============================================================================")
        parts.append("// UNPOLARIZED FUNCTIONS")
        parts.append("// ============================================================================")
        parts.append("")

    for func in unpol:
        # Per-function params: only include params actually used in computation (not asserts)
        filtered_body = '\n'.join(l for l in func['body'].split('\n') if 'assert(' not in l)
        fn_params = extract_params(filtered_body) if has_params else []
        rs = gen_function(func_name, func['level'], 'unpol', func['body'],
                         bool(fn_params), fn_params, is_vxc_only)
        parts.append(rs)
        parts.append("")

    if pol:
        parts.append("// ============================================================================")
        parts.append("// POLARIZED FUNCTIONS")
        parts.append("// ============================================================================")
        parts.append("")

    for func in pol:
        filtered_body = '\n'.join(l for l in func['body'].split('\n') if 'assert(' not in l)
        fn_params = extract_params(filtered_body) if has_params else []
        rs = gen_function(func_name, func['level'], 'pol', func['body'],
                         bool(fn_params), fn_params, is_vxc_only)
        parts.append(rs)
        parts.append("")

    src_dir = "lda_vxc" if is_vxc_only else "lda_exc"

    header = f"""//! {func_name.upper().replace('_', ' ')} kernel functions translated from libxc maple2c.
//!
//! Auto-translated from `libxc-master/src/maple2c/{src_dir}/{func_name}.c`.

{chr(10).join(imports)}
{pi_const}
{chr(10).join(parts)}"""

    return header


def main():
    if len(sys.argv) < 3:
        print("Usage: maple2c_translate.py <c_file> <output_rs_file> [--func-name NAME] [--vxc-only]")
        sys.exit(1)

    c_path = sys.argv[1]
    output_path = sys.argv[2]

    func_name = None
    is_vxc_only = '--vxc-only' in sys.argv

    for i, arg in enumerate(sys.argv):
        if arg == '--func-name' and i + 1 < len(sys.argv):
            func_name = sys.argv[i + 1]

    if func_name is None:
        func_name = Path(c_path).stem

    rust_code = translate_file(c_path, func_name, is_vxc_only)

    os.makedirs(os.path.dirname(output_path) or '.', exist_ok=True)
    with open(output_path, 'w') as f:
        f.write(rust_code)

    print(f"Translated {c_path} -> {output_path}")


if __name__ == '__main__':
    main()
