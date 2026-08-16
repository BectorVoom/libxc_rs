//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1469/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1469(t11665: f64, t11698: f64, t11683: f64, t11697: f64, t3577: f64, t11673: f64, t11678: f64, t11679: f64, t11687: f64, t11877: f64, t3576: f64, t11668: f64, t11674: f64, t11692: f64, t11741: f64, t11774: f64, t1227: f64, t15453: f64, t3243: f64, t3248: f64, t3490: f64, t3494: f64, t3516: f64, t3578: f64, t3580: f64, t42468: f64, t44953: f64, t44965: f64, t44968: f64, t44972: f64, t44976: f64, t4582: f64) -> f64 {
    let t44982 = t11665 * t11698;
    let t44985 = t3577 * t11697 * t11683;
    let t44988 = t3577 * t11697 * t11673;
    let t44991 = t11678 * t11697 * t11679;
    let t44994 = t3577 * t11697 * t11687;
    let t44996 = t11877 * t3576;
    let t44999 = -t11665 * t11674 / 384.0_f64 - 5.0_f64 / 2304.0_f64 * t11692 * t11668 * t3516 * t3243 + t44953 / 1728.0_f64 + 5.0_f64 / 2304.0_f64 * t3577 * t11668 * t3494 * t3243 + 5.0_f64 / 1152.0_f64 * t3490 * t11774 - 5.0_f64 / 864.0_f64 * t1227 * t4582 * t15453 * t42468 + t44965 * t11741 / 768.0_f64 + t44968 / 1728.0_f64 + t44972 / 3456.0_f64 + t44976 / 1728.0_f64 + t11692 * t3578 * t3516 * t3248 / 384.0_f64 - t44982 / 288.0_f64 - t44985 / 576.0_f64 - t44988 / 576.0_f64 - t44991 / 288.0_f64 - t44994 / 288.0_f64 - t44996 * t3580 / 384.0_f64;
    t44999
}
