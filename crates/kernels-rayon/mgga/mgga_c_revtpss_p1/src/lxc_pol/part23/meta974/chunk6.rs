//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3316/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3316(t1456: f64, t1458: f64, t1464: f64, t1914: f64, t1921: f64, t22533: f64, t22571: f64, t25049: f64, t25072: f64, t3: f64, t575: f64, t5790: f64, t5808: f64, t60620: f64, t60624: f64, t60629: f64, t6937: f64, t6951: f64, t75720: f64, t75727: f64, t75796: f64, t75808: f64, t86893: f64, t86897: f64, t86903: f64, t86909: f64, t86958: f64) -> f64 {
    let tv4rho43 = t3 * t575 * t86893 + t1456 * t25072 + t1458 * t86958 + t1464 * t25049 + 3.0_f64 * t1914 * t22571 + 3.0_f64 * t1921 * t22533 + 3.0_f64 * t5790 * t6951 + 3.0_f64 * t5808 * t6937 + 6.0_f64 * t60620 + 6.0_f64 * t60624 + 3.0_f64 * t60629 + 3.0_f64 * t75720 + 3.0_f64 * t75727 + 3.0_f64 * t75796 + t75808 + 3.0_f64 * t86897 + 3.0_f64 * t86903 + t86909;
    tv4rho43
}
