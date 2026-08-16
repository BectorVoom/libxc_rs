//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta179 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;
mod chunk8;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1137;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1138;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1139;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1140;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1141;
use chunk5::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1142;
use chunk6::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1143;
use chunk7::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1144;
use chunk8::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1145;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta179(t4295: f64, t829: f64, t235: f64, t4265: f64, t1499: f64, t1523: f64, t1525: f64, t226: f64, t255: f64, t2617: f64, t4162: f64, t4166: f64, t4281: f64, t4283: f64, t4286: f64, t4288: f64, t4291: f64, t4292: f64, t808: f64, t812: f64, t861: f64, t863: f64, t858: f64, t1528: f64, t259: f64, t2597: f64, t2713: f64, t4143: f64, t4145: f64, t4147: f64, t4149: f64, t4266: f64, t4268: f64, t4273: f64, t855: f64, t866: f64, t1530: f64, t2752: f64, t870: f64, t193: f64, t200: f64, t1484: f64, t262: f64, t1877: f64, t202: f64, t2373: f64, t2377: f64, t2522: f64, t4097: f64, t4099: f64, t4100: f64, t4103: f64, t4119: f64, t4198: f64, t4201: f64, t4204: f64, t4207: f64, t766: f64, t776: f64, t868: f64, t2523: f64, t2408: f64, t2417: f64, t2423: f64, t2426: f64, t2486: f64, t2518: f64, t2530: f64, t2537: f64, t2538: f64, t2665: f64, t4209: f64, t4213: f64, t4214: f64, t4215: f64, t4216: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t4296, t4298, t4300) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1137(t4295, t829, t235, t4265, t1499, t1523, t1525, t226, t255, t2617, t4162, t4166, t4281, t4283, t4286, t4288, t4291, t4292, t808, t812, t861, t863);
        let t4301 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1138(t4300, t858);
        let t4303 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1139(t1528, t259, t2597, t2713, t4143, t4145, t4147, t4149, t4266, t4268, t4273, t4301, t855, t866);
        let t4307 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1140(t1530, t2752);
        let t4310 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1141(t1530, t870);
        let t4314 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1142(t193, t200);
        let (t4315, t4319) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1143(t1484, t262, t1877, t193, t202, t2373, t2377, t2522, t4097, t4099, t4100, t4103, t4119, t4198, t4201, t4204, t4207, t4303, t4307, t4310, t4314, t766, t776, t868, t870);
        let t4323 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1144(t1484, t2523, t2408, t2417, t2423, t2426, t2486, t2518, t2522, t2530, t2537, t2538, t2665, t4209, t4213, t4214, t4215, t4216);
        let t4324 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1145(t4319, t4323);
    (t4296, t4298, t4300, t4301, t4303, t4307, t4310, t4314, t4315, t4324)
}
