//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1277/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1277(t37455: f64, t37468: f64, t39074: f64, t39075: f64, t39076: f64, t40411: f64, t42208: f64, t42209: f64, t42210: f64, t43875: f64, t43878: f64, t44574: f64, t44576: f64, t44579: f64, t44878: f64) -> f64 {
    let t44997 = -0.38422568777328955681e-2_f64 * t37455 - t44574 - t44576 + t44579 - 0.2881692658299671676e-2_f64 * t40411 + 0.1440846329149835838e-2_f64 * t43875 - 0.20496175532535769482e-3_f64 * t43878 + t39074 - t39075 - t39076 + t42208 - t42209 + t44878 + t42210 - 0.86737941314158990616e-4_f64 * t37468;
    t44997
}
