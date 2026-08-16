//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 562/1429 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk562(t3188: f64, t787: f64, t3187: f64, t283: f64, t462: f64, t2885: f64, t315: f64, t188: f64, t291: f64, t297: f64, t2531: f64, t799: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3189 = t3188 * t787;
    let t3190 = t3187 * t3189;
    let t3192 = t462 * t283;
    let t3193 = t2885 * t315;
    let t3194 = t3192 * t3193;
    let t3196 = t188 * t291;
    let t3197 = t3196 * t297;
    let t3198 = t3197 * t2531;
    let t3199 = t799 * t3198;
    (t3189, t3190, t3192, t3193, t3194, t3197, t3198, t3199)
}
