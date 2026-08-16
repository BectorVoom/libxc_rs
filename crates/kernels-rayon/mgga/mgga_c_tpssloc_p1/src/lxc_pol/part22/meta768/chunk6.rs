//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2608/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2608(t1734: f64, t6218: f64, t1089: f64, t11678: f64, t11692: f64, t1215: f64, t1227: f64, t15569: f64, t15659: f64, t15700: f64, t15701: f64, t15702: f64, t1735: f64, t18237: f64, t18321: f64, t18368: f64, t18395: f64, t18397: f64, t18401: f64, t3577: f64, t3578: f64, t4582: f64, t4729: f64, t4972: f64, t5046: f64, t52879: f64, t52903: f64, t5398: f64, t5979: f64, t607: f64, t6219: f64, t65464: f64, t65469: f64, t66120: f64, t70458: f64) -> (f64, f64) {
    let t72767 = t6218 * t1734;
    let t72783 = -t52903 * t18397 / 144.0_f64 + t11692 * t3578 * t65469 * t18395 / 1536.0_f64 - t11678 * t3578 * t15659 * t5979 * t1215 / 768.0_f64 + t11692 * t3578 * t15700 * t15701 * t5398 / 1536.0_f64 - t3577 * t3578 * t1735 * t18237 / 768.0_f64 - t3577 * t3578 * t6219 * t4729 / 768.0_f64 - t11678 * t3578 * t65464 * t1734 * t1089 * t607 / 768.0_f64 + t11692 * t3578 * t72767 * t15702 / 1536.0_f64 + t15569 * t18401 / 72.0_f64 - t66120 / 72.0_f64 - 11.0_f64 / 108.0_f64 * t18321 * t5046 - t52879 * t18368 / 768.0_f64 - t1227 * t4582 * t4972 * t70458 / 2304.0_f64;
    (t72767, t72783)
}
