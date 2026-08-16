//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 868/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk868(t2277: f64, t361: f64, t356: f64, t2281: f64, t6275: f64, t2196: f64, t828: f64, t2199: f64, t6143: f64, t852: f64, t2240: f64, t369: f64, t6121: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t6312 = 1.0_f64 / t2277 / t361;
    let t6313 = t356 * t6312;
    let t6314 = t6275 * t2281;
    let t6317 = t828 * t2196;
    let t6319 = 6.0_f64 * t6317 * t2199;
    let t6320 = t6143 * t852;
    let t6322 = 6.0_f64 * t2240 * t6320;
    let t6323 = t369 * t6121;
    (t6312, t6313, t6314, t6317, t6319, t6320, t6322, t6323)
}
