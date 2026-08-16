//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 1494/1505 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1494(t31032: f64, t31646: f64, t31649: f64, t109: f64, t1479: f64, t108: f64, t116912: f64, t31626: f64, t105875: f64, t117943: f64, t2: f64, t21872: f64, t21876: f64, t28036: f64, t31035: f64, t31287: f64, t31429: f64, t31433: f64, t4287: f64, t661: f64, t665: f64, t8258: f64, t8267: f64, t8311: f64, t8315: f64) -> f64 {
    let t118656 = t31032 * t31646;
    let t118658 = t31032 * t31649;
    let t118666 = t1479 * t109;
    let t118670 = t1479 * t108;
    let t118680 = t116912 * t31626;
    let t118688 = -t117943 + 10.0_f64 / 27.0_f64 * t118656 + 5.0_f64 / 9.0_f64 * t118658 - 5.0_f64 / 6.0_f64 * t8258 * t31429 * t4287 + t8258 * t8311 * t21876 / 4.0_f64 + 10.0_f64 / 9.0_f64 * t8258 * t118666 * t665 - 25.0_f64 / 27.0_f64 * t8267 * t118670 * t661 - 25.0_f64 / 36.0_f64 * t31287 * t31433 * t2 - 5.0_f64 / 24.0_f64 * t8267 * t8315 * t21872 + 2.0_f64 * t118680 - 3.0_f64 / 2.0_f64 * t31035 * t8311 * t105875 + 5.0_f64 / 2.0_f64 * t31035 * t31429 * t28036;
    t118688
}
