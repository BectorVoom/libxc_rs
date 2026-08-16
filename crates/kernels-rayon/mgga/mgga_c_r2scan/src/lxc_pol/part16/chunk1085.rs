//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 1085/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk1085(t11557: f64, t3261: f64, t5086: f64, t97: f64, t10609: f64, t1561: f64, t13908: f64, t986: f64, t3270: f64, t11584: f64, t37365: f64, t10673: f64, t11587: f64, t37360: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t39177 = 5.0_f64 / 8.0_f64 * t11557;
    let t39190 = t97 * t3261 * t5086;
    let t39197 = t97 * t10609 * t1561;
    let t39202 = t13908 * t986;
    let t39203 = t3270 * t39202;
    let t39215 = t37365 * t11584;
    let t39218 = t10673 * t11587 * t37360;
    (t39177, t39190, t39197, t39203, t39215, t39218)
}
