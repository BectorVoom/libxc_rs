//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta179 (260520-c91 hierarchical CSE).
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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1103;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1104;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1105;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1106;
use chunk4::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1107;
use chunk5::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1108;
use chunk6::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1109;
use chunk7::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1110;
use chunk8::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1111;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta179<F: Float>(t4295: F, t829: F, t235: F, t4265: F, t1499: F, t1523: F, t1525: F, t226: F, t255: F, t2617: F, t4162: F, t4166: F, t4281: F, t4283: F, t4286: F, t4288: F, t4291: F, t4292: F, t808: F, t812: F, t861: F, t863: F, t858: F, t1528: F, t259: F, t2597: F, t2713: F, t4143: F, t4145: F, t4147: F, t4149: F, t4266: F, t4268: F, t4273: F, t855: F, t866: F, t1530: F, t2752: F, t870: F, t193: F, t200: F, t1484: F, t262: F, t1877: F, t202: F, t2373: F, t2377: F, t2522: F, t4097: F, t4099: F, t4100: F, t4103: F, t4119: F, t4198: F, t4201: F, t4204: F, t4207: F, t766: F, t776: F, t868: F, t2523: F, t2408: F, t2417: F, t2423: F, t2426: F, t2486: F, t2518: F, t2530: F, t2537: F, t2538: F, t2665: F, t4209: F, t4213: F, t4214: F, t4215: F, t4216: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t4296, t4298, t4300) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1103::<F>(t4295, t829, t235, t4265, t1499, t1523, t1525, t226, t255, t2617, t4162, t4166, t4281, t4283, t4286, t4288, t4291, t4292, t808, t812, t861, t863);
        let t4301 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1104::<F>(t4300, t858);
        let t4303 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1105::<F>(t1528, t259, t2597, t2713, t4143, t4145, t4147, t4149, t4266, t4268, t4273, t4301, t855, t866);
        let t4307 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1106::<F>(t1530, t2752);
        let t4310 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1107::<F>(t1530, t870);
        let t4314 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1108::<F>(t193, t200);
        let (t4315, t4319) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1109::<F>(t1484, t262, t1877, t193, t202, t2373, t2377, t2522, t4097, t4099, t4100, t4103, t4119, t4198, t4201, t4204, t4207, t4303, t4307, t4310, t4314, t766, t776, t868, t870);
        let (t4320, t4323) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1110::<F>(t1484, t2523, t2408, t2417, t2423, t2426, t2486, t2518, t2522, t2530, t2537, t2538, t2665, t4209, t4213, t4214, t4215, t4216);
        let t4324 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1111::<F>(t4319, t4323);
    (t4296, t4298, t4300, t4301, t4303, t4307, t4310, t4314, t4315, t4320, t4324)
}
