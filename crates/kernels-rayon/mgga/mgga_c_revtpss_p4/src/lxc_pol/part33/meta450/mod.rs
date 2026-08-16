//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta450 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1637;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1638;
use chunk2::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1639;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta450(t471: f64, t5284: f64, t5332: f64, t3720: f64, t127: f64, t371: f64, t6645: f64, t1235: f64, t6609: f64, t3671: f64, t1208: f64, t6563: f64, t225: f64, t480: f64, t1238: f64, t17296: f64, t17298: f64, t17301: f64, t17304: f64, t17337: f64, t17609: f64, t1797: f64, t5274: f64, t5287: f64, t5293: f64, t5331: f64, t1248: f64, t6573: f64, t1250: f64, t19666: f64, t5302: f64, t1042: f64, t17550: f64, t19661: f64, t1715: f64, t17500: f64, t5056: f64, t5277: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t20838, t20842, t20843, t20846, t20847, t20849) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1637(t471, t5284, t5332, t3720, t127, t371, t6645, t1235, t6609, t3671, t1208, t6563);
        let (t20850, t20855) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1638(t20849, t225, t480, t1238, t17296, t17298, t17301, t17304, t17337, t17609, t1797, t20838, t20843, t20847, t5274, t5287, t5293, t5331);
        let (t20856, t20858, t20864, t20868, t20876, t20879) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1639(t1248, t6573, t1250, t3720, t19666, t5302, t1042, t17550, t19661, t1715, t17500, t5056, t5277);
    (t20838, t20842, t20846, t20849, t20850, t20855, t20856, t20858, t20864, t20868, t20876, t20879)
}
