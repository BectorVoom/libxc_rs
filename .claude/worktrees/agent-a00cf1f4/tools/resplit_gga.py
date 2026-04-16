#!/usr/bin/env python3
"""
Re-split GGA sub-crates using first-fit-decreasing bin packing.

Reads per-functional sizes from crates/kernel-gga/src/ (canonical source),
applies FFD bin packing with a 50K line target, creates ~22 sub-crates,
and updates the facade and workspace Cargo.toml.

The 25 deferred functionals (those with kxc_pol.rs or lxc_pol.rs > 5000 lines)
get #[cfg(feature = "order-kxc")] / #[cfg(feature = "order-lxc")] gates on
their kxc/lxc modules to prevent OOM during CubeCL proc macro expansion.

Usage:
    python3 tools/resplit_gga.py [--dry-run]
"""

import os
import re
import sys
import shutil

CRATES_DIR = "crates"
CANONICAL_SRC = os.path.join(CRATES_DIR, "kernel-gga", "src")
BIN_LIMIT = 50000
DEFERRED_THRESHOLD = 5000  # lines in kxc_pol.rs or lxc_pol.rs


def count_lines(path):
    """Count lines in a file."""
    with open(path, 'r') as f:
        return sum(1 for _ in f)


def get_functional_sizes():
    """Scan kernel-gga/src/ for all functional directories and measure line counts."""
    funcs = []  # list of (name, total_lines, is_deferred)

    for item in sorted(os.listdir(CANONICAL_SRC)):
        item_path = os.path.join(CANONICAL_SRC, item)
        if item == "lib.rs" or not os.path.isdir(item_path):
            continue

        total_lines = 0
        is_deferred = False
        for rs_file in sorted(os.listdir(item_path)):
            if rs_file.endswith('.rs'):
                fpath = os.path.join(item_path, rs_file)
                lines = count_lines(fpath)
                total_lines += lines
                if rs_file in ('kxc_pol.rs', 'lxc_pol.rs') and lines > DEFERRED_THRESHOLD:
                    is_deferred = True

        funcs.append((item, total_lines, is_deferred))

    return funcs


def bin_pack(funcs, limit):
    """First-fit decreasing bin packing."""
    sorted_funcs = sorted(funcs, key=lambda x: -x[1])

    bins = []       # list of lists of (name, lines, is_deferred)
    bin_totals = []

    for func in sorted_funcs:
        placed = False
        for i in range(len(bins)):
            if bin_totals[i] + func[1] <= limit:
                bins[i].append(func)
                bin_totals[i] += func[1]
                placed = True
                break
        if not placed:
            bins.append([func])
            bin_totals.append(func[1])

    # Sort bins by first functional name (alphabetically) for determinism
    bins.sort(key=lambda b: sorted(f[0] for f in b)[0])
    return bins


def has_deferred(bin_funcs):
    """Check if a bin contains any deferred functionals."""
    return any(f[2] for f in bin_funcs)


def rewrite_deferred_mod_rs(mod_rs_path):
    """Add cfg feature gates to kxc/lxc modules in a deferred functional's mod.rs."""
    with open(mod_rs_path, 'r') as f:
        lines = f.readlines()

    new_lines = []
    for line in lines:
        stripped = line.strip()
        if stripped in ('pub mod kxc_unpol;', 'pub mod kxc_pol;'):
            new_lines.append('#[cfg(feature = "order-kxc")]\n')
            new_lines.append(line)
        elif stripped in ('pub mod lxc_unpol;', 'pub mod lxc_pol;'):
            new_lines.append('#[cfg(feature = "order-lxc")]\n')
            new_lines.append(line)
        else:
            new_lines.append(line)

    with open(mod_rs_path, 'w') as f:
        f.writelines(new_lines)


def create_subcrate_cargo_toml(crate_dir, crate_num, needs_features):
    """Create Cargo.toml for a GGA sub-crate."""
    lines = [
        '[package]',
        f'name = "libxc-kernel-gga-{crate_num}"',
        'version = "0.1.0"',
        'edition = "2024"',
        '',
    ]

    if needs_features:
        lines += [
            '[features]',
            'order-kxc = []',
            'order-lxc = ["order-kxc"]',
            'all-orders = ["order-lxc"]',
            '',
        ]

    lines += [
        '[dependencies]',
        'cubecl = { version = "0.9.0", default-features = false, features = ["cpu"] }',
        'libxc-kernel-math = { path = "../kernel-math" }',
        '',
    ]

    with open(os.path.join(crate_dir, "Cargo.toml"), 'w') as f:
        f.write('\n'.join(lines))


def create_subcrate_lib_rs(crate_dir, func_names, crate_num):
    """Create lib.rs for a GGA sub-crate."""
    lines = [
        '#![allow(clippy::excessive_precision)]',
        '#![allow(clippy::needless_late_init)]',
        '#![allow(clippy::too_many_arguments)]',
        '',
        f'//! GGA kernel translations batch {crate_num}.',
        '',
    ]
    for name in sorted(func_names):
        lines.append(f'pub mod {name};')
    lines.append('')

    with open(os.path.join(crate_dir, "src", "lib.rs"), 'w') as f:
        f.write('\n'.join(lines))


def update_facade(num_bins, bins):
    """Rewrite kernel-gga/Cargo.toml and kernel-gga/src/lib.rs."""
    # -- Cargo.toml --
    cargo_lines = [
        '[package]',
        'name = "libxc-kernel-gga"',
        'version = "0.1.0"',
        'edition = "2024"',
        '',
    ]

    # Build feature forwarding: only forward to sub-crates that have features
    feature_crates = []
    for i, bin_funcs in enumerate(bins, 1):
        if has_deferred(bin_funcs):
            feature_crates.append(i)

    if feature_crates:
        cargo_lines.append('[features]')
        kxc_parts = ', '.join(f'"libxc-kernel-gga-{n}/order-kxc"' for n in feature_crates)
        lxc_parts = ', '.join(f'"libxc-kernel-gga-{n}/order-lxc"' for n in feature_crates)
        cargo_lines.append(f'order-kxc = [{kxc_parts}]')
        cargo_lines.append(f'order-lxc = ["order-kxc", {lxc_parts}]')
        cargo_lines.append('all-orders = ["order-lxc"]')
        cargo_lines.append('')

    cargo_lines.append('[dependencies]')
    cargo_lines.append('cubecl = { version = "0.9.0", default-features = false, features = ["cpu"] }')
    cargo_lines.append('libxc-kernel-math = { path = "../kernel-math" }')
    for i in range(1, num_bins + 1):
        cargo_lines.append(f'libxc-kernel-gga-{i} = {{ path = "../kernel-gga-{i}" }}')
    cargo_lines.append('')

    facade_dir = os.path.join(CRATES_DIR, "kernel-gga")
    with open(os.path.join(facade_dir, "Cargo.toml"), 'w') as f:
        f.write('\n'.join(cargo_lines))

    # -- lib.rs --
    lib_lines = [
        '#![allow(clippy::excessive_precision)]',
        '#![allow(clippy::needless_late_init)]',
        '#![allow(clippy::too_many_arguments)]',
        '',
        '//! GGA kernel translations from maple2c.',
        '//!',
        f'//! 131 GGA functionals total across {num_bins} sub-crates.',
        '//! Each sub-crate is sized via first-fit-decreasing bin packing to stay under',
        '//! ~50K lines of generated Rust, avoiding OOM during CubeCL proc macro expansion.',
        '',
        '// Re-export sub-crates containing compiled GGA functionals.',
    ]
    for i in range(1, num_bins + 1):
        lib_lines.append(f'pub use libxc_kernel_gga_{i} as batch{i};')
    lib_lines.append('')

    with open(os.path.join(facade_dir, "src", "lib.rs"), 'w') as f:
        f.write('\n'.join(lib_lines))


def update_workspace(num_bins):
    """Update root Cargo.toml workspace members: remove old GGA sub-crates, add new ones."""
    cargo_path = "Cargo.toml"
    with open(cargo_path, 'r') as f:
        content = f.read()

    # Remove old GGA sub-crate entries from workspace members
    # These are lines like:    "crates/kernel-gga-1",
    lines = content.split('\n')
    new_lines = []
    gga_sub_pattern = re.compile(r'^\s*"crates/kernel-gga-\d+"\s*,?\s*$')

    for line in lines:
        if gga_sub_pattern.match(line):
            continue  # Skip old GGA sub-crate entries
        new_lines.append(line)

    content = '\n'.join(new_lines)

    # Find the workspace members line with "crates/kernel-gga" (inside [workspace] section)
    # and insert new sub-crates after it
    lines = content.split('\n')
    new_lines = []
    in_workspace = False
    for line in lines:
        if line.strip() == '[workspace]':
            in_workspace = True
        elif line.strip().startswith('[') and in_workspace:
            in_workspace = False

        new_lines.append(line)

        # Only insert in workspace members section
        if in_workspace and '"crates/kernel-gga"' in line and 'kernel-gga-' not in line:
            for i in range(1, num_bins + 1):
                new_lines.append(f'    "crates/kernel-gga-{i}",')

    with open(cargo_path, 'w') as f:
        f.write('\n'.join(new_lines))


def main():
    dry_run = '--dry-run' in sys.argv

    # Step 1: Inventory and bin-pack
    print("Scanning crates/kernel-gga/src/ for functional directories...")
    funcs = get_functional_sizes()
    total_funcs = len(funcs)
    deferred_funcs = [f for f in funcs if f[2]]
    normal_funcs = [f for f in funcs if not f[2]]
    print(f"Found {total_funcs} functionals ({len(deferred_funcs)} deferred, {len(normal_funcs)} normal)")
    print(f"Deferred: {', '.join(f[0] for f in deferred_funcs)}")

    # Step 2: Bin pack
    bins = bin_pack(funcs, BIN_LIMIT)
    num_bins = len(bins)
    print(f"\nBin packing result: {num_bins} sub-crates (target max {BIN_LIMIT} lines)")

    for i, bin_funcs in enumerate(bins, 1):
        total = sum(f[1] for f in bin_funcs)
        has_def = has_deferred(bin_funcs)
        flag = " [FEATURES: order-kxc/order-lxc]" if has_def else ""
        names = ', '.join(sorted(f[0] for f in bin_funcs))
        print(f"  gga-{i:2d}: {total:6d} lines ({len(bin_funcs):2d} funcs){flag}: {names}")

    if dry_run:
        print("\nDry run -- no changes made.")
        return

    # Step 3: Delete old sub-crates
    print("\nRemoving old GGA sub-crates...")
    for entry in sorted(os.listdir(CRATES_DIR)):
        if re.match(r'^kernel-gga-\d+$', entry):
            path = os.path.join(CRATES_DIR, entry)
            shutil.rmtree(path)
            print(f"  Removed {entry}")

    # Step 4: Create new sub-crates
    print(f"\nCreating {num_bins} new sub-crates...")
    for i, bin_funcs in enumerate(bins, 1):
        crate_name = f"kernel-gga-{i}"
        crate_dir = os.path.join(CRATES_DIR, crate_name)
        src_dir = os.path.join(crate_dir, "src")
        os.makedirs(src_dir, exist_ok=True)

        needs_features = has_deferred(bin_funcs)

        # Create Cargo.toml
        create_subcrate_cargo_toml(crate_dir, i, needs_features)

        # Copy functional directories from canonical source
        func_names = []
        for func_name, lines, is_deferred in bin_funcs:
            src = os.path.join(CANONICAL_SRC, func_name)
            dst = os.path.join(src_dir, func_name)
            shutil.copytree(src, dst)
            func_names.append(func_name)

            # Add feature gates on deferred functional mod.rs
            if is_deferred:
                rewrite_deferred_mod_rs(os.path.join(dst, "mod.rs"))

        # Create lib.rs
        create_subcrate_lib_rs(crate_dir, func_names, i)

        total = sum(f[1] for f in bin_funcs)
        print(f"  Created {crate_name}: {len(func_names)} funcs, {total} lines{' [features]' if needs_features else ''}")

    # Step 5: Update facade
    print("\nUpdating kernel-gga facade...")
    update_facade(num_bins, bins)

    # Step 6: Update workspace Cargo.toml
    print("Updating workspace Cargo.toml...")
    update_workspace(num_bins)

    # Summary
    total_lines = sum(f[1] for f in funcs)
    print(f"\n{'='*60}")
    print(f"RESPLIT COMPLETE: {total_funcs} functionals -> {num_bins} sub-crates")
    print(f"Total lines: {total_lines}")
    print(f"Average per crate: {total_lines // num_bins}")
    print(f"Deferred functionals with feature gates: {len(deferred_funcs)}")
    print(f"{'='*60}")


if __name__ == '__main__':
    main()
