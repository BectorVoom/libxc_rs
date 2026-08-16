//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta209 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1045;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1046;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1047;
use chunk3::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1048;
use chunk4::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1049;
use chunk5::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1050;
use chunk6::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1051;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta209(t1629: f64, t4684: f64, t1022: f64, t1625: f64, t1060: f64, t383: f64, t4657: f64, t1003: f64, t1058: f64, t1061: f64, t1063: f64, t1610: f64, t1630: f64, t1632: f64, t3180: f64, t3186: f64, t3200: f64, t353: f64, t384: f64, t4615: f64, t4669: f64, t4674: f64, t4678: f64, t4681: f64, t1055: f64, t1052: f64, t1066: f64, t1635: f64, t3026: f64, t3169: f64, t388: f64, t4553: f64, t4555: f64, t4557: f64, t4559: f64, t4658: f64, t4660: f64, t4665: f64, t193: f64, t336: f64, t1637: f64, t3216: f64, t1068: f64, t1070: f64, t4353: f64, t4356: f64, t4358: f64, t4361: f64, t4398: f64, t4402: f64, t4480: f64, t4482: f64, t4485: f64, t4487: f64, t4491: f64, t4495: f64, t4500: f64, t25: f64, t265: f64, t394: f64, t4324: f64, t1074: f64, t1408: f64, t1409: f64, t1534: f64, t1642: f64, t396: f64, t3966: f64, t40: f64, t4332: f64, t606: f64, t607: f64, t873: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64, t1654: f64, t690: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t4685, t4688, t4689, t4691, t4693) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1045(t1629, t4684, t1022, t1625, t1060, t383, t4657, t1003, t1058, t1061, t1063, t1610, t1630, t1632, t3180, t3186, t3200, t353, t384, t4615, t4669, t4674, t4678, t4681);
        let t4694 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1046(t1055, t4693);
        let t4696 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1047(t1052, t1066, t1635, t3026, t3169, t388, t4553, t4555, t4557, t4559, t4658, t4660, t4665, t4694);
        let t4700 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1048(t193, t336);
        let (t4701, t4704) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1049(t1637, t3216, t1068, t1070, t193, t336, t4353, t4356, t4358, t4361, t4398, t4402, t4480, t4482, t4485, t4487, t4491, t4495, t4500, t4696, t4700);
        let (t4705, t4712) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1050(t25, t265, t394, t4324, t4704, t1074, t1408, t1409, t1534, t1642, t396, t3966, t40, t4332, t606, t607, t873, dens_threshold, rho0, zeta_threshold);
        let t4721 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1051(t1654, t690);
    (t4685, t4688, t4689, t4691, t4693, t4694, t4696, t4700, t4701, t4705, t4712, t4721)
}
