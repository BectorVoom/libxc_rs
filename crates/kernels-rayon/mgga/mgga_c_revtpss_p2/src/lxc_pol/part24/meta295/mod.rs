//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta295 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1078;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1079;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1080;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta295(t3153: f64, t6622: f64, t1263: f64, t6587: f64, t3172: f64, t6624: f64, t1247: f64, t1032: f64, t6564: f64, t1246: f64, t127: f64, t371: f64, t6645: f64, t1235: f64, t6609: f64, t3671: f64, t1208: f64, t6563: f64, t225: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t20800, t20809, t20816, t20817, t20819, t20820, t20842) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1078(t3153, t6622, t1263, t6587, t3172, t6624, t1247, t1032, t6564, t1246, t127, t371, t6645);
        let (t20843, t20846, t20847, t20849) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1079(t1235, t20842, t127, t371, t6609, t3671, t1208, t6563);
        let t20850 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1080(t20849, t225);
    (t20800, t20809, t20816, t20817, t20819, t20820, t20842, t20843, t20846, t20847, t20849, t20850)
}
