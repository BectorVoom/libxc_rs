//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta192 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk925;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk926;
use chunk2::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk927;
use chunk3::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk928;
use chunk4::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk929;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta192<F: Float>(t4282: F, t829: F, t1519: F, t814: F, t235: F, t4265: F, t1499: F, t1523: F, t1525: F, t226: F, t255: F, t2617: F, t4162: F, t4166: F, t4281: F, t4283: F, t4286: F, t4288: F, t4291: F, t808: F, t812: F, t861: F, t863: F, t858: F, t1528: F, t259: F, t2597: F, t2713: F, t4143: F, t4145: F, t4147: F, t4149: F, t4266: F, t4268: F, t4273: F, t855: F, t866: F, t1530: F, t2752: F, t870: F, t193: F, t200: F, t1484: F, t262: F, t1877: F, t202: F, t2373: F, t2377: F, t2522: F, t4097: F, t4099: F, t4100: F, t4103: F, t4119: F, t4198: F, t4201: F, t4204: F, t4207: F, t766: F, t776: F, t868: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t4292, t4295, t4296, t4298, t4300) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk925::<F>(t4282, t829, t1519, t814, t235, t4265, t1499, t1523, t1525, t226, t255, t2617, t4162, t4166, t4281, t4283, t4286, t4288, t4291, t808, t812, t861, t863);
        let t4301 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk926::<F>(t4300, t858);
        let t4303 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk927::<F>(t1528, t259, t2597, t2713, t4143, t4145, t4147, t4149, t4266, t4268, t4273, t4301, t855, t866);
        let (t4307, t4310, t4314) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk928::<F>(t1530, t2752, t870, t193, t200);
        let (t4315, t4319) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk929::<F>(t1484, t262, t1877, t193, t202, t2373, t2377, t2522, t4097, t4099, t4100, t4103, t4119, t4198, t4201, t4204, t4207, t4303, t4307, t4310, t4314, t766, t776, t868, t870);
    (t4292, t4295, t4296, t4298, t4300, t4301, t4303, t4307, t4310, t4314, t4315, t4319)
}
