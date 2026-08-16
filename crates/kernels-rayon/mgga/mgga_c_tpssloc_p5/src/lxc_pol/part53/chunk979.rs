//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 979/1059 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk979(t31058: f64, t7458: f64, t19456: f64, t8327: f64, t4028: f64, t12725: f64, t20173: f64, t33193: f64, t3941: f64, t4072: f64, t8326: f64, t16524: f64, t31285: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t120721 = 2.0_f64 * t7458 * t31058;
    let t120728 = 2.0_f64 * t19456 * t8327;
    let t120730 = 2.0_f64 * t4028 * t31058;
    let t120735 = 2.0_f64 * t12725 * t8327;
    let t120800 = 27.0_f64 * t20173 * t33193;
    let t120803 = 27.0_f64 * t3941 * t8326 * t4072;
    let t120807 = 27.0_f64 * t16524 * t31285;
    (t120721, t120728, t120730, t120735, t120800, t120803, t120807)
}
