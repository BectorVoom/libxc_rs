//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 730/1226 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk730(t2710: f64, t798: f64, t116: f64, t229: f64, t212: f64, t776: f64, t2586: f64, t210: f64, t214: f64, t9516: f64, t597: f64, t60: f64) -> (f64, f64, f64, f64, f64) {
    let t9520 = t798 * t2710;
    let t9523 = t229 * t116;
    let t9524 = t212 * t776;
    let t9525 = t9523 * t9524;
    let t9526 = t2586 * t9525;
    let t9529 = t210 * t214 * t9516;
    let t9533 = 1.0_f64 / t60 / t597;
    (t9520, t9523, t9526, t9529, t9533)
}
