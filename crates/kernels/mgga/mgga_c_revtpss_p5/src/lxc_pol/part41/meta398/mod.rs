//! MGGA_C_REVTPSS lxc pol kernel — _part41_v4rho3tau_4 meta398 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1352;
use chunk1::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1353;
use chunk2::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1354;
use chunk3::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1355;
use chunk4::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1356;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_meta398<F: Float>(t3172: F, t6618: F, t3711: F, t6634: F, t3610: F, t5265: F, t5293: F, t19680: F, t5302: F, t1042: F, t3153: F, t6628: F, t5352: F, t3720: F, t6622: F, t5341: F, t5333: F, t1263: F, t6587: F, t1122: F, t6624: F, t1247: F, t1032: F, t6564: F, t1246: F, t1214: F, t5819: F, t1252: F, t1261: F, t12809: F, t17547: F, t1797: F, t5331: F, t5340: F, t471: F, t5284: F, t5332: F, t127: F, t371: F, t6645: F, t1235: F, t6609: F, t3671: F, t1208: F, t6563: F, t225: F, t480: F, t1238: F, t17296: F, t17298: F, t17301: F, t17304: F, t17337: F, t17609: F, t5274: F, t5287: F) -> (F, F, F, F, F, F, F) {
        let (t20784, t20787, t20789, t20792, t20795) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1352::<F>(t3172, t6618, t3711, t6634, t3610, t5265, t5293, t19680, t5302, t1042, t3153, t6628);
        let (t20797, t20800, t20802, t20806, t20811, t20816) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1353::<F>(t20795, t5352, t3720, t3153, t6622, t5341, t5333, t1263, t6587, t1122, t1042, t3172, t6624);
        let (t20823, t20828) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1354::<F>(t1247, t20816, t1032, t6564, t1246, t1214, t5819, t5302, t1042, t1252, t1261, t12809, t17547, t1797, t20784, t20787, t20789, t20792, t20797, t20802, t20806, t20811, t3711, t5331, t5340);
        let (t20838, t20843, t20847, t20849) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1355::<F>(t471, t5284, t5332, t3720, t127, t371, t6645, t1235, t6609, t3671, t1208, t6563);
        let (t20850, t20855) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1356::<F>(t20849, t225, t480, t1238, t17296, t17298, t17301, t17304, t17337, t17609, t1797, t20838, t20843, t20847, t5274, t5287, t5293, t5331);
    (t20795, t20800, t20823, t20828, t20849, t20850, t20855)
}
