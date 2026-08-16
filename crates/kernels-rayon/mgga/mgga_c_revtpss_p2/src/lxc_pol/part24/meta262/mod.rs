//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta262 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1032;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta262(t17395: f64, t3717: f64, t1284: f64, t5219: f64, t3624: f64, t12879: f64, t1715: f64, t247: f64, t1261: f64, t1803: f64, t3670: f64, t5436: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t17396, t17400, t17401, t17416, t17417, t17438, t17448) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1032(t17395, t3717, t1284, t5219, t3624, t12879, t1715, t247, t1261, t1803, t3670, t5436);
    (t17396, t17400, t17401, t17416, t17417, t17438, t17448)
}
