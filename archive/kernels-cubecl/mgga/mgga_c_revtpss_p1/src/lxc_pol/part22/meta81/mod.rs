//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta81 (260520-c91 hierarchical CSE).
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
mod chunk9;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk585;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk586;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk587;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk588;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk589;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk590;
use chunk6::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk591;
use chunk7::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk592;
use chunk8::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk593;
use chunk9::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk594;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta81<F: Float>(t1598: F, t1612: F, t1638: F, t1640: F, t1644: F, t373: F, t1045: F, t1042: F, t1066: F, t1592: F, t247: F, t1009: F, t1011: F, t1025: F, t1041: F, t1060: F, t1063: F, t1656: F, t1660: F, t1665: F, t375: F, t225: F, t385: F, t1082: F, t1651: F, t1089: F, t378: F, t380: F, t1024: F, t1087: F, t1647: F, t342: F, t381: F, t1079: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t1668 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk585::<F>(t1598, t1612, t1638, t1640, t1644);
        let (t1670, t1671) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk586::<F>(t1668, t373, t1045, t1042);
        let t1675 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk587::<F>(t1066, t1592, t247);
        let t1678 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk588::<F>(t1009, t1011, t1025, t1041, t1060, t1063, t1656, t1660, t1665, t1671, t1675, t375);
        let (t1679, t1680) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk589::<F>(t1678, t225, t385);
        let t1685 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk590::<F>(t1082, t1651);
        let t1689 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk591::<F>(t1089, t1668, t378);
        let t1692 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk592::<F>(t1678, t380);
        let t1695 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk593::<F>(t1024, t1087, t1647, t1685, t1689, t1692, t342, t381);
        let t1696 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk594::<F>(t1079, t1695);
    (t1668, t1670, t1671, t1675, t1678, t1679, t1680, t1685, t1689, t1692, t1695, t1696)
}
