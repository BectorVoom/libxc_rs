//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3163/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3163(t1213: f64, t18941: f64, t248: f64, t3570: f64, t15730: f64, t5019: f64, t1216: f64, t3966: f64, t1227: f64, t1230: f64, t15495: f64, t15498: f64, t15708: f64, t15710: f64, t15740: f64, t1737: f64, t1748: f64, t19051: f64, t3527: f64, t3531: f64, t3577: f64, t3578: f64, t3585: f64, t44929: f64, t44932: f64, t4728: f64, t5014: f64, t5030: f64, t53406: f64, t53507: f64, t5971: f64, t6227: f64, t6232: f64, t63357: f64, t63363: f64) -> (f64, f64) {
    let t65424 = t1213 * t248 * t3570 * t18941;
    let t65444 = t5019 * t15730;
    let t65452 = t1216 * t3966;
    let t65463 = t65424 / 2304.0_f64 - t1227 * t248 * t1230 * t63363 / 1152.0_f64 + t53507 * t1748 / 432.0_f64 + t15498 * t5030 / 216.0_f64 + t44929 * t6227 / 1536.0_f64 - t19051 * t3527 / 4608.0_f64 - t19051 * t3531 / 2304.0_f64 - t53406 * t1737 / 288.0_f64 - t15495 * t5014 / 144.0_f64 + t65444 / 1296.0_f64 + 5.0_f64 / 6912.0_f64 * t1227 * t248 * t3585 * t63357 - t44932 * t6232 / 3072.0_f64 - t3577 * t3578 * t4728 * t65452 / 576.0_f64 - t3577 * t3578 * t5971 * t15708 / 384.0_f64 - t15740 * t15710 / 576.0_f64;
    (t65452, t65463)
}
