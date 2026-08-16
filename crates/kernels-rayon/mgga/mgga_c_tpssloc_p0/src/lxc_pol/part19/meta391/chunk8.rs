//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1479/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1479(t11159: f64, t11539: f64, t1174: f64, t1090: f64, t11154: f64, t11546: f64, t11668: f64, t11678: f64, t11722: f64, t11855: f64, t11863: f64, t1196: f64, t1216: f64, t1227: f64, t3243: f64, t3248: f64, t3252: f64, t3440: f64, t3490: f64, t3494: f64, t3509: f64, t3536: f64, t3577: f64, t3578: f64, t39097: f64, t39110: f64, t42374: f64, t43711: f64, t43715: f64, t43732: f64, t45192: f64, t45197: f64, t45211: f64, t45222: f64, t45224: f64, t4582: f64, t4972: f64, t974: f64) -> f64 {
    let t45227 = t1174 * t11539 * t11159;
    let t45246 = -t1174 * t974 * t1196 * t39110 / 288.0_f64 - t1174 * t974 * t45192 * t39097 / 12.0_f64 - t45197 * t3578 * t11722 * t1090 / 192.0_f64 - t3577 * t3578 * t3494 * t3252 / 768.0_f64 - t3577 * t3578 * t3494 * t3248 / 384.0_f64 + 5.0_f64 / 1728.0_f64 * t45211 - t3490 * t11863 / 192.0_f64 - t1227 * t4582 * t4972 * t42374 / 576.0_f64 + t3536 * t11855 / 768.0_f64 - t45222 / 36.0_f64 - t45224 / 2304.0_f64 + t45227 / 54.0_f64 + t1174 * t3440 * t43715 / 54.0_f64 - 7.0_f64 / 108.0_f64 * t1174 * t11546 * t43732 + t1174 * t3440 * t43711 / 6.0_f64 + 5.0_f64 / 1152.0_f64 * t11678 * t11668 * t3509 * t3243 + 5.0_f64 / 576.0_f64 * t3577 * t11668 * t1216 * t11154;
    t45246
}
