//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1079/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1079<F: Float>(t1882: F, t23216: F, t23304: F, t23158: F, t23285: F, t23371: F, t23267: F, t8392: F, t23271: F, t23315: F, t23236: F, t23161: F, t103: F, t22862: F, t23332: F, t1334: F, t3281: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t91912 = t1882 * t23216;
    let t91926 = t1882 * t23304;
    let t91928 = t1882 * t23158;
    let t91945 = t1882 * t23285;
    let t91951 = t1882 * t23371;
    let t91980 = t8392 * t23267;
    let t91982 = t8392 * t23271;
    let t91993 = t1882 * t23315;
    let t92006 = t8392 * t23236;
    let t92014 = t1882 * t23161;
    let t92016 = t103 * t22862;
    let t92021 = t8392 * t23332;
    let t92024 = 28.0 / 81.0 * t3281 * t1334;
    (t91912, t91926, t91928, t91945, t91951, t91980, t91982, t91993, t92006, t92014, t92016, t92021, t92024)
}
