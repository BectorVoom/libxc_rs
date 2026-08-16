//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2446/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2446(t3082: f64, t4617: f64, t3132: f64, t607: f64, t3120: f64, t4594: f64, t10904: f64, t14025: f64, t10403: f64, t10408: f64, t1041: f64, t10937: f64, t13975: f64, t13980: f64, t13991: f64, t14009: f64, t14230: f64, t1539: f64, t2960: f64, t3070: f64, t3071: f64, t3130: f64, t42334: f64, t42522: f64, t43241: f64, t4337: f64, t4342: f64, t4582: f64, t4583: f64, t4596: f64, t45997: f64, t48506: f64) -> (f64, f64) {
    let t49993 = t4617 * t3082;
    let t49994 = t49993 / 4608.0_f64;
    let t50009 = t3132 * t607;
    let t50014 = t4594 * t3120;
    let t50027 = t10904 * t14025;
    let t50035 = -t49994 - 3.0_f64 / 512.0_f64 * t42334 * t13991 + t3130 * t4582 * t48506 * t4594 / 512.0_f64 + t3130 * t4582 * t13975 * t13980 / 512.0_f64 - t3070 * t3071 * t4342 * t43241 / 768.0_f64 - t10403 * t3071 * t4342 * t50009 / 384.0_f64 + t10403 * t3071 * t1539 * t50014 / 768.0_f64 + 5.0_f64 / 2304.0_f64 * t10403 * t10408 * t4337 * t50009 + t10937 * t14230 / 72.0_f64 + 19.0_f64 / 288.0_f64 * t42522 * t4596 - t50027 / 72.0_f64 - t1041 * t4582 * t4583 * t45997 / 768.0_f64 + 2.0_f64 / 9.0_f64 * t2960 * t14009;
    (t50014, t50035)
}
