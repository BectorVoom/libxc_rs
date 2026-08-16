//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 543/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk543(t3114: f64, t352: f64, t355: f64, t922: f64, t721: f64, t839: f64, t1060: f64, t1059: f64, t1068: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t3115 = t352 * t3114;
    let t3116 = t355 * t922;
    let t3117 = t3116 * t721;
    let t3118 = t3115 * t3117;
    let t3120 = t355 * t839;
    let t3121 = t3120 * t721;
    let t3122 = t1060 * t3121;
    let t3124 = t1068 * t1059;
    (t3115, t3117, t3118, t3121, t3122, t3124)
}
