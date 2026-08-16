//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta432 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1627;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1628;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1629;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1630;
use chunk4::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1631;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta432(t1261: f64, t12925: f64, t3172: f64, t12921: f64, t3711: f64, t3617: f64, t675: f64, t247: f64, t3363: f64, t3609: f64, t44169: f64, t1263: f64, t215: f64, t1122: f64, t12776: f64, t12788: f64, t12812: f64, t12835: f64, t12866: f64, t12867: f64, t12936: f64, t3613: f64, t3718: f64, t372: f64, t3720: f64, t44431: f64, t44658: f64, t44661: f64, t44664: f64, t44672: f64, t44675: f64, t5352: f64, t12772: f64, t12846: f64, t5331: f64, t3625: f64, t12780: f64, t1121: f64, t13045: f64, t606: f64, t13052: f64, t13054: f64, t11262: f64, t3713: f64, t3584: f64, t3588: f64, t1250: f64, t12787: f64, t12803: f64, t12810: f64, t12840: f64, t12847: f64, t13102: f64, t17429: f64, t17638: f64, t17644: f64, t17688: f64, t17709: f64, t17747: f64, t17753: f64, t3626: f64, t3629: f64, t3647: f64, t44501: f64, t3601: f64, t12657: f64, t1284: f64, t3624: f64, t12875: f64, t12916: f64, t12871: f64, t5340: f64, t3568: f64, t1222: f64, t12282: f64, t17471: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t44678, t44681, t44696, t44698, t44701) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1627(t1261, t12925, t3172, t12921, t3711, t3617, t675, t247, t3363, t3609, t44169, t1263, t215);
        let t44706 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1628(t1122, t1261, t247, t44701, t12776, t12788, t12812, t12835, t12866, t12867, t12936, t3613, t3718, t372, t3720, t44431, t44658, t44661, t44664, t44672, t44675, t44678, t44681, t44696, t44698, t5352);
        let (t44711, t44726, t44729, t44738, t44748) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1629(t12772, t12846, t5331, t12776, t3625, t12780, t1121, t13045, t606, t13052, t13054, t3172);
        let (t44753, t44758) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1630(t11262, t3711, t3713, t3584, t3588, t1250, t12787, t12803, t12810, t12840, t12847, t13102, t17429, t17638, t17644, t17688, t17709, t17747, t17753, t3625, t3626, t3629, t3647, t3718, t3720, t44501, t44711, t44726, t44729, t44738, t44748, t5331);
        let (t44759, t44769, t44773, t44776, t44778, t44786) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1631(t3584, t3601, t12657, t1284, t3624, t12875, t12916, t5331, t12871, t5340, t3568, t1222, t12282, t17471);
    (t44706, t44753, t44758, t44759, t44769, t44773, t44776, t44778, t44786)
}
