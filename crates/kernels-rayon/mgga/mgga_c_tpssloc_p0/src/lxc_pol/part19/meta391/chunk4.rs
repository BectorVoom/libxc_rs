//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1475/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1475(t1090: f64, t11148: f64, t11163: f64, t11172: f64, t11665: f64, t11670: f64, t11729: f64, t11739: f64, t11809: f64, t11825: f64, t11853: f64, t1216: f64, t1218: f64, t1227: f64, t1230: f64, t248: f64, t3490: f64, t3531: f64, t3577: f64, t3578: f64, t43800: f64, t43804: f64, t45080: f64, t45086: f64, t45102: f64, t45108: f64, t45112: f64, t45114: f64, t45119: f64, t45126: f64, t45128: f64) -> f64 {
    let t45133 = -t1227 * t248 * t1230 * t43804 / 4608.0_f64 - t3490 * t11809 / 192.0_f64 - t1227 * t248 * t1230 * t43800 / 192.0_f64 + t45080 * t1218 / 768.0_f64 + 5.0_f64 / 1152.0_f64 * t11665 * t11670 + t45086 / 576.0_f64 - t3577 * t3578 * t11172 * t1216 / 1152.0_f64 - t3577 * t3578 * t11163 * t1216 / 192.0_f64 - t3577 * t3578 * t11853 * t1090 / 1152.0_f64 + t45102 / 1152.0_f64 - t11825 * t3531 / 384.0_f64 - t45108 / 288.0_f64 - t45112 + t45114 * t3578 * t11729 * t1090 / 192.0_f64 - t45119 * t3578 * t11739 * t1090 / 1152.0_f64 + 5.0_f64 / 1728.0_f64 * t45126 - 5.0_f64 / 1296.0_f64 * t3577 * t45128 * t11148 * t1216;
    t45133
}
