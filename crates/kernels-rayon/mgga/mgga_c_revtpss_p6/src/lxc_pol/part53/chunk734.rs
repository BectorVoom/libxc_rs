//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 734/1244 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk734(t5: f64, t72: f64, t7714: f64, t1927: f64, t1493: f64, t76: f64, t1926: f64, t1923: f64, t1928: f64, t6958: f64, t7702: f64, t7706: f64, t7709: f64) -> (f64, f64, f64, f64, f64) {
    let t7 = piecewise3(0.0_f64 < t5, t5, -t5);
    let t8 = -t7 <= -0.999999999999e0_f64;
    let t7715 = t7714 * t72;
    let t7716 = t7715 * t1927;
    let t7719 = t76 * t1493;
    let t7720 = t1926 * t7719;
    let t7724 = piecewise3(t8, 0.0_f64, -t7702 * t1928 / 6.0_f64 + 5.0_f64 / 6.0_f64 * t6958 * t7706 + t7709 * t1928 / 3.0_f64 - t1923 * t7716 / 6.0_f64 - t1923 * t7720 / 6.0_f64);
    (t7715, t7716, t7719, t7720, t7724)
}
