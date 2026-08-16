//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1161/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1161(t1065: f64, t39197: f64, t42878: f64, t42877: f64, t792: f64, t39190: f64, t795: f64, t37327: f64, t4176: f64, t14656: f64, t986: f64, t3270: f64) -> (f64, f64, f64, f64) {
    let t42881 = 15.0_f64 / 4.0_f64 * t39197 * t1065 * t42878;
    let t42882 = t42877 * t792;
    let t42885 = 135.0_f64 / 32.0_f64 * t39190 * t1065 * t42882;
    let t42886 = t42877 * t795;
    let t42889 = 15.0_f64 / 8.0_f64 * t37327 * t4176 * t42886;
    let t42890 = t14656 * t986;
    let t42891 = t3270 * t42890;
    (t42881, t42885, t42889, t42891)
}
