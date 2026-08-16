//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2597/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2597(t1730: f64, t19032: f64, t1017: f64, t1207: f64, t1210: f64, t22173: f64, t372: f64, t471: f64, t479: f64, t15507: f64, t19095: f64, t1218: f64, t1232: f64, t65660: f64, t65662: f64, t65664: f64, t65668: f64, t65670: f64, t65672: f64, t65674: f64, t65676: f64, t65681: f64) -> f64 {
    let t72384 = t1730 * t19032;
    let t72389 = t1207 * t1210 * t22173 * t1017;
    let t72398 = t471 * t479 * t22173 * t372;
    let t72403 = t15507 * t19095;
    let t72405 = t65660 / 768.0_f64 + 5.0_f64 / 6912.0_f64 * t65662 - 19.0_f64 / 2592.0_f64 * t65664 - 19.0_f64 / 864.0_f64 * t72384 * t1232 - 209.0_f64 / 2592.0_f64 * t72389 * t1218 + t65668 / 216.0_f64 + 19.0_f64 / 864.0_f64 * t65670 - 19.0_f64 / 1296.0_f64 * t65672 - t65674 / 1536.0_f64 + 209.0_f64 / 3888.0_f64 * t72398 * t1232 - t65676 / 1152.0_f64 + t65681 / 1536.0_f64 + t72403 / 288.0_f64;
    t72405
}
