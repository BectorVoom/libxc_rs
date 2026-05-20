//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta432 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1627;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1628;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1629;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1630;
use chunk4::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1631;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta432<F: Float>(t1261: F, t12925: F, t3172: F, t12921: F, t3711: F, t3617: F, t675: F, t247: F, t3363: F, t3609: F, t44169: F, t1263: F, t215: F, t1122: F, t12776: F, t12788: F, t12812: F, t12835: F, t12866: F, t12867: F, t12936: F, t3613: F, t3718: F, t372: F, t3720: F, t44431: F, t44658: F, t44661: F, t44664: F, t44672: F, t44675: F, t5352: F, t12772: F, t12846: F, t5331: F, t3625: F, t12780: F, t1121: F, t13045: F, t606: F, t13052: F, t13054: F, t11262: F, t3713: F, t3584: F, t3588: F, t1250: F, t12787: F, t12803: F, t12810: F, t12840: F, t12847: F, t13102: F, t17429: F, t17638: F, t17644: F, t17688: F, t17709: F, t17747: F, t17753: F, t3626: F, t3629: F, t3647: F, t44501: F, t3601: F, t12657: F, t1284: F, t3624: F, t12875: F, t12916: F, t12871: F, t5340: F, t3568: F, t1222: F, t12282: F, t17471: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t44678, t44681, t44696, t44698, t44701) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1627::<F>(t1261, t12925, t3172, t12921, t3711, t3617, t675, t247, t3363, t3609, t44169, t1263, t215);
        let t44706 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1628::<F>(t1122, t1261, t247, t44701, t12776, t12788, t12812, t12835, t12866, t12867, t12936, t3613, t3718, t372, t3720, t44431, t44658, t44661, t44664, t44672, t44675, t44678, t44681, t44696, t44698, t5352);
        let (t44711, t44726, t44729, t44738, t44748) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1629::<F>(t12772, t12846, t5331, t12776, t3625, t12780, t1121, t13045, t606, t13052, t13054, t3172);
        let (t44753, t44758) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1630::<F>(t11262, t3711, t3713, t3584, t3588, t1250, t12787, t12803, t12810, t12840, t12847, t13102, t17429, t17638, t17644, t17688, t17709, t17747, t17753, t3625, t3626, t3629, t3647, t3718, t3720, t44501, t44711, t44726, t44729, t44738, t44748, t5331);
        let (t44759, t44769, t44773, t44776, t44778, t44786) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1631::<F>(t3584, t3601, t12657, t1284, t3624, t12875, t12916, t5331, t12871, t5340, t3568, t1222, t12282, t17471);
    (t44706, t44753, t44758, t44759, t44769, t44773, t44776, t44778, t44786)
}
