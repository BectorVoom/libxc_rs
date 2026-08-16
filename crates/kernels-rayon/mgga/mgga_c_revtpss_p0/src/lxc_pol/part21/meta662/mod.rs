//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta662 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2457;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta662(t11626: f64, t358: f64, t3145: f64, t3153: f64, t3154: f64, t11268: f64, t3173: f64, t1063: f64, t11232: f64, t3172: f64, t11982: f64, t11285: f64, t3127: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t42862, t42865, t42871, t42872, t42883, t42886, t42889, t42892) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2457(t11626, t358, t3145, t3153, t3154, t11268, t3173, t1063, t11232, t3172, t11982, t11285, t3127);
    (t42862, t42865, t42871, t42872, t42883, t42886, t42889, t42892)
}
