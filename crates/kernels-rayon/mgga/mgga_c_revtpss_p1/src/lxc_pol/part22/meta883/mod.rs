//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta883 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3057;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta883(t2444: f64, t4534: f64, t689: f64, t198: f64, t2394: f64, t4567: f64, t588: f64, t15183: f64, t698: f64, t15172: f64, t2439: f64, t4625: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t51759, t51780, t51835, t51909, t51911, t51913) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3057(t2444, t4534, t689, t198, t2394, t4567, t588, t15183, t698, t15172, t2439, t4625);
    (t51759, t51780, t51835, t51909, t51911, t51913)
}
