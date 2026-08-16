//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 744/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk744(t70082: f64, t14563: f64, t2160: f64, t638: f64, t14559: f64, t70237: f64, t14580: f64, t899: f64, t70328: f64, t70376: f64, t70385: f64, t70439: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t71672 = 0.30487649791575028312e-3_f64 * t70082;
    let t71717 = t638 * t2160 * t14563;
    let t71720 = t638 * t2160 * t14559;
    let t71744 = 0.60975299583150056624e-3_f64 * t70237;
    let t71772 = t899 * t14580;
    let t71789 = 0.3830813990396805546e-3_f64 * t70328;
    let t71802 = 0.162600798888400151e-2_f64 * t70376;
    let t71804 = 0.32526727992809621482e-4_f64 * t70385;
    let t71832 = 0.2316441583394736328e-4_f64 * t70439;
    (t71672, t71717, t71720, t71744, t71772, t71789, t71802, t71804, t71832)
}
