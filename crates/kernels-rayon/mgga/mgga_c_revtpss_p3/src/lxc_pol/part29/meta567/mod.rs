//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta567 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1914;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta567(t25207: f64, t98651: f64, t1468: f64, t2411: f64, t14365: f64, t1544: f64, t2257: f64, t4433: f64, t890: f64, t27383: f64, t61155: f64, t27375: f64, t92790: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t98652, t98659, t98662, t98674, t98675, t98688, t98694) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1914(t25207, t98651, t1468, t2411, t14365, t1544, t2257, t4433, t890, t27383, t61155, t27375, t92790);
    (t98652, t98659, t98662, t98674, t98675, t98688, t98694)
}
