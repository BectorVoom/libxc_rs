//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1291/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1291(t17942: f64, t517: f64, t1215: f64, t18436: f64, t3251: f64, t339: f64, t5719: f64, t790: f64, t3277: f64, t18464: f64, t3350: f64, t2376: f64, t5726: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t60730 = t17942 * t517;
    let t60731 = t60730 * t1215;
    let t60733 = t18436 * t3251;
    let t60738 = t339 * t5719 * t790;
    let t60739 = t60738 * t3277;
    let t60744 = t18464 * t3350;
    let t60749 = t339 * t5726 * t2376;
    (t60730, t60731, t60733, t60738, t60739, t60744, t60749)
}
