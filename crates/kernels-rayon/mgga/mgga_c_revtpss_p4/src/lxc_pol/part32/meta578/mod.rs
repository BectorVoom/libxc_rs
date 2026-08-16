//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta578 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1905;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta578(t2435: f64, t8099: f64, t25904: f64, t26231: f64, t97802: f64, t26234: f64, t98041: f64, t102244: f64, t94674: f64, t97700: f64, t102268: f64, t102165: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t102315, t102316, t102320, t102324, t102325, t102329, t102339, t102346) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1905(t2435, t8099, t25904, t26231, t97802, t26234, t98041, t102244, t94674, t97700, t102268, t102165);
    (t102315, t102316, t102320, t102324, t102325, t102329, t102339, t102346)
}
