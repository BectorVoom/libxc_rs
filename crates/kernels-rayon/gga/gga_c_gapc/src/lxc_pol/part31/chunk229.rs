//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 229/1447 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk229(t260: f64, t786: f64, t154: f64, t276: f64, t299: f64, t311: f64, t751: f64, t837: f64, t841: f64, t845: f64, t869: f64, t871: f64) -> (f64, f64) {
    let t872 = t260 * t786;
    let t875 = 0.14341111111111111111e-1_f64 * t154 * t837 * t276 + 0.21511666666666666667e-1_f64 * t154 * t841 * t276 - 0.21511666666666666667e-1_f64 * t154 * t299 * t845 - t869 * t260 + t871 * t872 - t311 * t751;
    (t872, t875)
}
