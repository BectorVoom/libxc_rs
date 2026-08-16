//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta411 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1521;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta411(t3154: f64, t42871: f64, t1036: f64, t11240: f64, t42646: f64, t11268: f64, t3173: f64, t1063: f64, t11232: f64, t3172: f64, t11982: f64, t11285: f64, t3127: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t42872, t42873, t42879, t42883, t42886, t42889, t42892) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1521(t3154, t42871, t1036, t11240, t42646, t11268, t3173, t1063, t11232, t3172, t11982, t11285, t3127);
    (t42872, t42873, t42879, t42883, t42886, t42889, t42892)
}
