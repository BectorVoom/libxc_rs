//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta450 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1637;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1638;
use chunk2::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1639;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta450<F: Float>(t471: F, t5284: F, t5332: F, t3720: F, t127: F, t371: F, t6645: F, t1235: F, t6609: F, t3671: F, t1208: F, t6563: F, t225: F, t480: F, t1238: F, t17296: F, t17298: F, t17301: F, t17304: F, t17337: F, t17609: F, t1797: F, t5274: F, t5287: F, t5293: F, t5331: F, t1248: F, t6573: F, t1250: F, t19666: F, t5302: F, t1042: F, t17550: F, t19661: F, t1715: F, t17500: F, t5056: F, t5277: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t20838, t20842, t20843, t20846, t20847, t20849) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1637::<F>(t471, t5284, t5332, t3720, t127, t371, t6645, t1235, t6609, t3671, t1208, t6563);
        let (t20850, t20855) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1638::<F>(t20849, t225, t480, t1238, t17296, t17298, t17301, t17304, t17337, t17609, t1797, t20838, t20843, t20847, t5274, t5287, t5293, t5331);
        let (t20856, t20858, t20864, t20868, t20876, t20879) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1639::<F>(t1248, t6573, t1250, t3720, t19666, t5302, t1042, t17550, t19661, t1715, t17500, t5056, t5277);
    (t20838, t20842, t20846, t20849, t20850, t20855, t20856, t20858, t20864, t20868, t20876, t20879)
}
