//! MGGA_C_REVTPSS lxc pol kernel — _part40_v4rho3tau_3 meta84 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk488;
use chunk1::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk489;
use chunk2::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk490;
use chunk3::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk491;
use chunk4::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk492;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_meta84(t1678: f64, t225: f64, t385: f64, t1082: f64, t1651: f64, t1089: f64, t1668: f64, t378: f64, t380: f64, t1024: f64, t1087: f64, t1647: f64, t342: f64, t381: f64, t1079: f64, t265: f64, t393: f64, t1076: f64, t1652: f64, t386: f64, t995: f64, t1102: f64, t1587: f64, t1598: f64, t1612: f64, t1638: f64, t1640: f64, t1644: f64, t198: f64, t336: f64, t30: f64, t1468: f64, t1469: f64, t395: f64, t45: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64, t1121: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t1680, t1685, t1689, t1692, t1695) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk488(t1678, t225, t385, t1082, t1651, t1089, t1668, t378, t380, t1024, t1087, t1647, t342, t381);
        let t1696 = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk489(t1079, t1695);
        let (t1699, t1704) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk490(t265, t393, t1076, t1647, t1652, t1680, t1696, t342, t386, t995, t1102, t1587, t1598, t1612, t1638, t1640, t1644, t198, t336);
        let (t1709, t1711) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk491(t30, t1468, t1469, t1587, t1704, t265, t395, t45, dens_threshold, rho0, zeta_threshold);
        let t1715 = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk492(t1121, t1469);
    (t1680, t1685, t1689, t1692, t1695, t1696, t1699, t1704, t1709, t1711, t1715)
}
