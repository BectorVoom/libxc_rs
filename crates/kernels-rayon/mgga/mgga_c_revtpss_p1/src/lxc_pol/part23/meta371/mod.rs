//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta371 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1700;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta371(t3172: f64, t4874: f64, t3127: f64, t4802: f64, t1063: f64, t4807: f64, t3153: f64, t4866: f64, t11922: f64, t4911: f64, t3115: f64, t1032: f64, t4743: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t15769, t15771, t15772, t15774, t15775, t15776, t15780, t15794, t15796, t15816) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1700(t3172, t4874, t3127, t4802, t1063, t4807, t3153, t4866, t11922, t4911, t3115, t1032, t4743);
    (t15769, t15771, t15772, t15774, t15775, t15776, t15780, t15794, t15796, t15816)
}
