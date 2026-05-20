//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta655 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2605;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2606;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2607;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta655<F: Float>(t471: F, t5284: F, t5332: F, t3720: F, t127: F, t371: F, t6645: F, t1235: F, t6609: F, t3671: F, t1208: F, t6563: F, t225: F, t480: F, t1238: F, t17296: F, t17298: F, t17301: F, t17304: F, t17337: F, t17609: F, t1797: F, t5274: F, t5287: F, t5293: F, t5331: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t20836, t20837, t20838, t20842, t20843, t20846, t20847, t20849) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2605::<F>(t471, t5284, t5332, t3720, t127, t371, t6645, t1235, t6609, t3671, t1208, t6563);
        let t20850 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2606::<F>(t20849, t225);
        let (t20851, t20855) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2607::<F>(t20850, t480, t1238, t17296, t17298, t17301, t17304, t17337, t17609, t1797, t20838, t20843, t20847, t5274, t5287, t5293, t5331);
    (t20836, t20837, t20838, t20842, t20846, t20849, t20850, t20851, t20855)
}
