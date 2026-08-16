//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta193 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1173;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1174;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1175;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1176;
use chunk4::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1177;
use chunk5::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1178;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta193<F: Float>(t1060: F, t4680: F, t1022: F, t1932: F, t360: F, t1629: F, t1625: F, t383: F, t4657: F, t1003: F, t1058: F, t1061: F, t1063: F, t1610: F, t1630: F, t1632: F, t3180: F, t3186: F, t3200: F, t353: F, t384: F, t4615: F, t4669: F, t4674: F, t4678: F, t1055: F, t1052: F, t1066: F, t1635: F, t3026: F, t3169: F, t388: F, t4553: F, t4555: F, t4557: F, t4559: F, t4658: F, t4660: F, t4665: F, t193: F, t336: F, t1637: F, t3216: F, t1068: F, t1070: F, t4353: F, t4356: F, t4358: F, t4361: F, t4398: F, t4402: F, t4480: F, t4482: F, t4485: F, t4487: F, t4491: F, t4495: F, t4500: F, t25: F, t265: F, t394: F, t4324: F, t1074: F, t1408: F, t1409: F, t1534: F, t1642: F, t396: F, t3966: F, t40: F, t4332: F, t606: F, t607: F, t873: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t4681, t4684) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1173::<F>(t1060, t4680, t1022, t1932, t360);
        let (t4685, t4689, t4691, t4693) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1174::<F>(t1629, t4684, t1022, t1625, t1060, t383, t4657, t1003, t1058, t1061, t1063, t1610, t1630, t1632, t3180, t3186, t3200, t353, t384, t4615, t4669, t4674, t4678, t4681);
        let (t4694, t4696) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1175::<F>(t1055, t4693, t1052, t1066, t1635, t3026, t3169, t388, t4553, t4555, t4557, t4559, t4658, t4660, t4665);
        let t4700 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1176::<F>(t193, t336);
        let (t4701, t4704) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1177::<F>(t1637, t3216, t1068, t1070, t193, t336, t4353, t4356, t4358, t4361, t4398, t4402, t4480, t4482, t4485, t4487, t4491, t4495, t4500, t4696, t4700);
        let (t4705, t4712) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1178::<F>(t25, t265, t394, t4324, t4704, t1074, t1408, t1409, t1534, t1642, t396, t3966, t40, t4332, t606, t607, t873, dens_threshold, rho0, zeta_threshold);
    (t4681, t4684, t4685, t4689, t4691, t4693, t4694, t4696, t4700, t4701, t4705, t4712)
}
