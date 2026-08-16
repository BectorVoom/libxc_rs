//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 655/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk655(t3333: f64, t3359: f64, t3236: f64, t3238: f64, t3245: f64, t3250: f64, t3254: f64) -> (f64, f64) {
    let t3360 = t3333 * t3359;
    let t3363 = 0.12361111111111111111e-1_f64 * t3236;
    let t3368 = t3363 - 0.61805555555555555556e-2_f64 * t3238 - 0.61805555555555555555e-2_f64 * t3245 + 0.18541666666666666667e-1_f64 * t3250 + 0.92708333333333333333e-2_f64 * t3254;
    (t3360, t3368)
}
