//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta390 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1466;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1467;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1468;
use chunk3::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1469;
use chunk4::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1470;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta390(t11661: f64, t13969: f64, t3506: f64, t11721: f64, t3493: f64, t11858: f64, t1226: f64, t3030: f64, t3481: f64, t3032: f64, t3505: f64, t3514: f64, t1174: f64, t11760: f64, t135: f64, t11147: f64, t3439: f64, t11719: f64, t11724: f64, t11728: f64, t11734: f64, t11770: f64, t11814: f64, t1214: f64, t1216: f64, t1227: f64, t1230: f64, t1232: f64, t15620: f64, t248: f64, t3496: f64, t3508: f64, t3511: f64, t3515: f64, t3518: f64, t39097: f64, t43757: f64, t44668: f64, t44873: f64, t44879: f64, t44886: f64, t44890: f64, t44894: f64, t44896: f64, t4582: f64, t974: f64, t11789: f64, t820: f64, t3577: f64, t3579: f64, t11737: f64, t44857: f64, t11791: f64, t3490: f64, t3252: f64, t3248: f64, t11665: f64, t11698: f64, t11683: f64, t11697: f64, t11673: f64, t11678: f64, t11679: f64, t11687: f64, t11877: f64, t3576: f64, t11668: f64, t11674: f64, t11692: f64, t11741: f64, t11774: f64, t15453: f64, t3243: f64, t3494: f64, t3516: f64, t3578: f64, t3580: f64, t42468: f64, t11647: f64, t1203: f64, t11859: f64, t1222: f64, t11797: f64, t11172: f64, t3521: f64, t11801: f64, t204: f64, t486: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t44904, t44906, t44918, t44927, t44929, t44932) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1466(t11661, t13969, t3506, t11721, t3493, t11858, t1226, t3030, t3481, t3032, t3505, t3514);
        let t44943 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1467(t1174, t11760, t135, t11147, t3439, t11719, t11724, t11728, t11734, t11770, t11814, t1214, t1216, t1227, t1230, t1232, t15620, t248, t3496, t3506, t3508, t3511, t3515, t3518, t39097, t43757, t44668, t44873, t44879, t44886, t44890, t44894, t44896, t44904, t44906, t44918, t44929, t44932, t4582, t974);
        let (t44953, t44965, t44968, t44972, t44976) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1468(t11789, t820, t3577, t3579, t11737, t44857, t11791, t3490, t1227, t248, t3252, t3248);
        let t44999 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1469(t11665, t11698, t11683, t11697, t3577, t11673, t11678, t11679, t11687, t11877, t3576, t11668, t11674, t11692, t11741, t11774, t1227, t15453, t3243, t3248, t3490, t3494, t3516, t3578, t3580, t42468, t44953, t44965, t44968, t44972, t44976, t4582);
        let (t45002, t45007, t45009, t45013, t45015, t45017) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1470(t11647, t1203, t11859, t1222, t11797, t3490, t11172, t1227, t248, t3521, t11801, t204, t486);
    (t44927, t44943, t44999, t45002, t45007, t45009, t45013, t45015, t45017)
}
