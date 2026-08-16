//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2617/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2617(t1215: f64, t5011: f64, t1222: f64, t15765: f64, t3242: f64, t3448: f64, t11728: f64, t13969: f64, t15630: f64, t11678: f64, t11722: f64, t1174: f64, t1177: f64, t11825: f64, t1227: f64, t15560: f64, t15617: f64, t1653: f64, t3490: f64, t3509: f64, t3578: f64, t45086: f64, t45102: f64, t45162: f64, t45197: f64, t4582: f64, t45993: f64, t46006: f64, t4733: f64, t4972: f64, t4987: f64, t5030: f64, t50879: f64) -> (f64, f64, f64, f64) {
    let t53176 = t5011 * t1215;
    let t53185 = t15765 * t1222;
    let t53187 = t3448 * t3242;
    let t53220 = t11728 * t13969 * t15630;
    let t53236 = -t1174 * t1177 * t50879 / 12.0_f64 - t3490 * t15617 / 256.0_f64 - t1227 * t4582 * t4972 * t46006 / 768.0_f64 + 5.0_f64 / 13824.0_f64 * t1227 * t4582 * t4987 * t45993 - t53220 / 256.0_f64 + t45086 / 2304.0_f64 - t45162 * t15560 / 768.0_f64 - t11678 * t3578 * t4733 * t3509 / 768.0_f64 - t45197 * t3578 * t1653 * t11722 / 768.0_f64 - t11825 * t5030 / 1536.0_f64 + t45102 / 4608.0_f64;
    (t53176, t53185, t53187, t53236)
}
