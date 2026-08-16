//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1472/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1472(t3584: f64, t676: f64, t1227: f64, t248: f64, t3243: f64, t11159: f64, t11665: f64, t11668: f64, t11678: f64, t11684: f64, t11721: f64, t1174: f64, t1177: f64, t11805: f64, t1214: f64, t1216: f64, t15620: f64, t15661: f64, t15708: f64, t2250: f64, t3247: f64, t3490: f64, t3508: f64, t3577: f64, t3578: f64, t42374: f64, t43723: f64, t44699: f64, t45002: f64, t45007: f64, t45009: f64, t45013: f64, t45015: f64, t45020: f64, t45027: f64, t45030: f64, t45037: f64, t45044: f64, t4582: f64, t4987: f64) -> f64 {
    let t45046 = t676 * t3584;
    let t45049 = t1227 * t248 * t45046 * t3243;
    let t45066 = -t11665 * t11684 / 384.0_f64 + t45002 / 2592.0_f64 - t1174 * t1177 * t43723 / 36.0_f64 + t45007 / 1152.0_f64 - t45009 / 576.0_f64 - t45013 / 1728.0_f64 - t45015 / 288.0_f64 + t45020 / 2592.0_f64 + 5.0_f64 / 3456.0_f64 * t1227 * t4582 * t4987 * t42374 - t45027 / 288.0_f64 - 3.0_f64 / 256.0_f64 * t45030 * t248 * t1214 * t44699 * t11721 + 7.0_f64 / 1536.0_f64 * t45037 * t248 * t1214 * t44699 * t3508 - 5.0_f64 / 972.0_f64 * t45044 - 5.0_f64 / 10368.0_f64 * t45049 - t3577 * t3578 * t3247 * t2250 * t15708 / 192.0_f64 + 5.0_f64 / 1152.0_f64 * t3577 * t11668 * t1216 * t11159 - t11678 * t3578 * t15620 * t15661 / 192.0_f64 - t3490 * t11805 / 1152.0_f64;
    t45066
}
