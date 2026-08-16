//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2598/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2598(t22298: f64, t486: f64, t11668: f64, t11678: f64, t11692: f64, t15659: f64, t1735: f64, t18232: f64, t19000: f64, t19033: f64, t3577: f64, t3578: f64, t45037: f64, t45114: f64, t45197: f64, t4582: f64, t4724: f64, t4729: f64, t4974: f64, t4978: f64, t4984: f64, t6225: f64, t6230: f64, t65464: f64, t65474: f64, t65545: f64, t65689: f64, t65691: f64, t72146: f64) -> (f64, f64) {
    let t72445 = t486 * t22298;
    let t72452 = 5.0_f64 / 2304.0_f64 * t11678 * t11668 * t6225 * t4724 - 5.0_f64 / 4608.0_f64 * t11692 * t11668 * t6230 * t4724 - t45197 * t3578 * t65474 * t19000 / 256.0_f64 + t45114 * t3578 * t6225 * t19000 / 256.0_f64 - t11678 * t3578 * t6225 * t4729 / 384.0_f64 - t11678 * t3578 * t65464 * t19000 / 768.0_f64 - t11678 * t3578 * t15659 * t72146 / 384.0_f64 + 5.0_f64 / 4608.0_f64 * t3577 * t11668 * t1735 * t18232 + t65689 / 3456.0_f64 - 11.0_f64 / 324.0_f64 * t65691 - 19.0_f64 / 576.0_f64 * t65545 * t4984 + 7.0_f64 / 1536.0_f64 * t45037 * t4582 * t72445 * t4978 - 19.0_f64 / 432.0_f64 * t19033 * t4974;
    (t72445, t72452)
}
