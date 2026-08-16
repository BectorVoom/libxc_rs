//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2614/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2614(t11697: f64, t22161: f64, t3577: f64, t19025: f64, t5001: f64, t1090: f64, t11668: f64, t1174: f64, t1218: f64, t15569: f64, t15594: f64, t1735: f64, t18215: f64, t18368: f64, t18590: f64, t18969: f64, t22299: f64, t3578: f64, t44621: f64, t45044: f64, t45119: f64, t5024: f64, t52628: f64, t53162: f64, t6211: f64, t66334: f64, t66337: f64, t71164: f64) -> f64 {
    let t72959 = t3577 * t11697 * t22161;
    let t72967 = t5001 * t19025;
    let t72970 = 5.0_f64 / 6912.0_f64 * t66334 - t66337 / 1152.0_f64 - t45119 * t3578 * t22299 * t1090 / 4608.0_f64 - 5.0_f64 / 3888.0_f64 * t45044 + 35.0_f64 / 972.0_f64 * t1174 * t44621 * t71164 - t15594 * t6211 / 768.0_f64 + 5.0_f64 / 2304.0_f64 * t3577 * t11668 * t1735 * t18215 - t72959 / 2304.0_f64 + t53162 + t5024 * t18590 / 72.0_f64 + t15569 * t18969 / 288.0_f64 + t52628 * t18368 / 144.0_f64 + 19.0_f64 / 576.0_f64 * t72967 * t1218;
    t72970
}
