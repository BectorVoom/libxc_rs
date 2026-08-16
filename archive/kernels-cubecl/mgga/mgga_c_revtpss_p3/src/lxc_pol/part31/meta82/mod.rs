//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta82 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk523;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk524;
use chunk2::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk525;
use chunk3::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk526;
use chunk4::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk527;
use chunk5::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk528;
use chunk6::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk529;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta82<F: Float>(t1066: F, t1592: F, t247: F, t1009: F, t1011: F, t1025: F, t1041: F, t1060: F, t1063: F, t1656: F, t1660: F, t1665: F, t1671: F, t375: F, t225: F, t385: F, t1082: F, t1651: F, t1089: F, t1668: F, t378: F, t380: F, t1024: F, t1087: F, t1647: F, t342: F, t381: F, t1079: F, t1076: F, t1652: F, t386: F, t995: F, t30: F, t265: F, t393: F, t1102: F, t1587: F, t1598: F, t1612: F, t1638: F, t1640: F, t1644: F, t198: F, t336: F, t1468: F, t1469: F, t395: F, t45: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t1675 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk523::<F>(t1066, t1592, t247);
        let t1678 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk524::<F>(t1009, t1011, t1025, t1041, t1060, t1063, t1656, t1660, t1665, t1671, t1675, t375);
        let (t1680, t1685, t1689, t1692, t1695) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk525::<F>(t1678, t225, t385, t1082, t1651, t1089, t1668, t378, t380, t1024, t1087, t1647, t342, t381);
        let t1696 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk526::<F>(t1079, t1695);
        let t1699 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk527::<F>(t1076, t1647, t1652, t1680, t1696, t342, t386, t995);
        let (t1704, t1709) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk528::<F>(t30, t265, t393, t1102, t1587, t1598, t1612, t1638, t1640, t1644, t1699, t198, t336, t1468, t1469, t395, t45, dens_threshold, rho0, zeta_threshold);
        let t1711 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk529::<F>(t1468);
    (t1675, t1678, t1680, t1685, t1689, t1692, t1695, t1696, t1699, t1704, t1709, t1711)
}
