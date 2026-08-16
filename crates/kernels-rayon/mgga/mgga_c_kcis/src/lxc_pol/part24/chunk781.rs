//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 781/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk781(t4552: f64, t4992: f64, t86: f64, t1014: f64, t4789: f64, t2820: f64, t4557: f64, t4807: f64, t9429: f64, t2861: f64, t4778: f64, t4797: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t13199 = t86 * t4992 * t4552;
    let t13238 = t1014 * t4789;
    let t13241 = t86 * t2820 * t4552;
    let t13242 = t13241 * t4557;
    let t13243 = 0.3684876543209876543e-2_f64 * t13242;
    let t13270 = t9429 * t4807;
    let t13271 = 0.14739506172839506172e-2_f64 * t13270;
    let t13277 = t2861 * t4778;
    let t13278 = 0.33163888888888888888e-2_f64 * t13277;
    let t13301 = t9429 * t4797;
    (t13199, t13238, t13242, t13243, t13270, t13271, t13277, t13278, t13301)
}
