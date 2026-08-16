//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 971/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk971(t11351: f64, t3560: f64, t885: f64, t10656: f64, t10923: f64, t10932: f64, t10944: f64, t10947: f64, t10678: f64, t10685: f64, t10692: f64, t10695: f64, t10937: f64, t11205: f64, t11328: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t11352 = t11351 / 4.0_f64;
    let t11353 = t3560 * t885;
    let t11354 = 2.0_f64 * t11353;
    let t11357 = 0.30487649791575028312e-3_f64 * t10656;
    let t11364 = 0.30487649791575028312e-3_f64 * t10923;
    let t11365 = 0.86737941314158990616e-4_f64 * t10932;
    let t11367 = 0.60975299583150056624e-3_f64 * t10944;
    let t11368 = 0.162600798888400151e-2_f64 * t10947;
    let t11369 = t11205 - 0.20496175532535769482e-3_f64 * t10678 + 0.1440846329149835838e-2_f64 * t10685 + t11328 - 0.72042316457491791901e-3_f64 * t10692 - 0.1440846329149835838e-2_f64 * t10695 + t11364 - t11365 + 0.3842256877732895568e-2_f64 * t10937 + t11367 + t11368;
    (t11352, t11354, t11357, t11364, t11365, t11367, t11368, t11369)
}
