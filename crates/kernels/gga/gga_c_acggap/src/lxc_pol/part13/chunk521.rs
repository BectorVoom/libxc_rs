//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 521/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk521<F: Float>(t3114: F, t352: F, t355: F, t922: F, t721: F, t839: F, t1060: F, t1059: F, t1068: F, t1072: F, t301: F, t21: F, t5: F) -> (F, F, F, F, F, F, F) {
    let t3115 = t352 * t3114;
    let t3116 = t355 * t922;
    let t3117 = t3116 * t721;
    let t3118 = t3115 * t3117;
    let t3120 = t355 * t839;
    let t3121 = t3120 * t721;
    let t3122 = t1060 * t3121;
    let t3124 = t1068 * t1059;
    let t3125 = t1072 * t301;
    let t3126 = t21 * t5;
    (t3116, t3118, t3120, t3122, t3124, t3125, t3126)
}
