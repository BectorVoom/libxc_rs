//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta157 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk975;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk976;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk977;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk978;
use chunk4::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk979;
use chunk5::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk980;
use chunk6::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk981;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta157<F: Float>(t193: F, t336: F, t1637: F, t3216: F, t1068: F, t1070: F, t4353: F, t4356: F, t4358: F, t4361: F, t4398: F, t4402: F, t4480: F, t4482: F, t4485: F, t4487: F, t4491: F, t4495: F, t4500: F, t4696: F, t25: F, t265: F, t394: F, t4324: F, t1074: F, t1408: F, t1409: F, t1534: F, t1642: F, t396: F, t3966: F, t40: F, t4332: F, t606: F, t607: F, t873: F, dens_threshold: F, rho0: F, zeta_threshold: F, t1654: F, t690: F, t3242: F, t3240: F, t123: F, t3247: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let t4700 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk975::<F>(t193, t336);
        let (t4701, t4704) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk976::<F>(t1637, t3216, t1068, t1070, t193, t336, t4353, t4356, t4358, t4361, t4398, t4402, t4480, t4482, t4485, t4487, t4491, t4495, t4500, t4696, t4700);
        let (t4705, t4712) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk977::<F>(t25, t265, t394, t4324, t4704, t1074, t1408, t1409, t1534, t1642, t396, t3966, t40, t4332, t606, t607, t873, dens_threshold, rho0, zeta_threshold);
        let t4721 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk978::<F>(t1654, t690);
        let t4723 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk979::<F>(t1409, t3242);
        let t4724 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk980::<F>(t4723, t607);
        let (t4725, t4726, t4728, t4729) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk981::<F>(t3240, t4724, t123, t1409, t3247, t607);
    (t4700, t4701, t4705, t4712, t4721, t4723, t4724, t4725, t4726, t4728, t4729)
}
