//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta120 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk657;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta120(t125: f64, t1558: f64, t1544: f64, t854: f64, t236: f64, t807: f64, t1469: f64, t2375: f64, t2382: f64, t1532: f64, t750: f64, t1534: f64, t177: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t4365, t4371, t4372, t4373, t4377, t4384, t4397, t4398) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk657(t125, t1558, t1544, t854, t236, t807, t1469, t2375, t2382, t1532, t750, t1534, t177);
    (t4365, t4371, t4372, t4373, t4377, t4384, t4397, t4398)
}
