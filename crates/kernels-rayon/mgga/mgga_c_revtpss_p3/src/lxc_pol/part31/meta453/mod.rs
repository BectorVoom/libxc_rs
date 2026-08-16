//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta453 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1627;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1628;
use chunk2::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1629;
use chunk3::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1630;
use chunk4::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1631;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta453(t3172: f64, t6618: f64, t3711: f64, t6634: f64, t3610: f64, t5265: f64, t5293: f64, t19680: f64, t5302: f64, t1042: f64, t3153: f64, t6628: f64, t5352: f64, t3720: f64, t6622: f64, t5341: f64, t5333: f64, t1263: f64, t6587: f64, t1122: f64, t6624: f64, t1247: f64, t1032: f64, t6564: f64, t1246: f64, t1214: f64, t5819: f64, t1252: f64, t1261: f64, t12809: f64, t17547: f64, t1797: f64, t5331: f64, t5340: f64, t471: f64, t5284: f64, t5332: f64, t127: f64, t371: f64, t6645: f64, t1235: f64, t6609: f64, t3671: f64, t1208: f64, t6563: f64, t225: f64, t480: f64, t1238: f64, t17296: f64, t17298: f64, t17301: f64, t17304: f64, t17337: f64, t17609: f64, t5274: f64, t5287: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t20784, t20787, t20789, t20792, t20795) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1627(t3172, t6618, t3711, t6634, t3610, t5265, t5293, t19680, t5302, t1042, t3153, t6628);
        let (t20797, t20800, t20802, t20806, t20811, t20816) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1628(t20795, t5352, t3720, t3153, t6622, t5341, t5333, t1263, t6587, t1122, t1042, t3172, t6624);
        let (t20823, t20828) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1629(t1247, t20816, t1032, t6564, t1246, t1214, t5819, t5302, t1042, t1252, t1261, t12809, t17547, t1797, t20784, t20787, t20789, t20792, t20797, t20802, t20806, t20811, t3711, t5331, t5340);
        let (t20838, t20843, t20847, t20849) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1630(t471, t5284, t5332, t3720, t127, t371, t6645, t1235, t6609, t3671, t1208, t6563);
        let (t20850, t20855) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1631(t20849, t225, t480, t1238, t17296, t17298, t17301, t17304, t17337, t17609, t1797, t20838, t20843, t20847, t5274, t5287, t5293, t5331);
    (t20795, t20800, t20823, t20828, t20849, t20850, t20855)
}
