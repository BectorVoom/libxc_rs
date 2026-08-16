//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta705 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2456;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2457;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta705(t47371: f64, t786: f64, t10115: f64, t1441: f64, t4093: f64, t9292: f64, t1432: f64, t1433: f64, t39497: f64, t10111: f64, t1428: f64, t588: f64, t10022: f64, t2453: f64, t268: f64, t39644: f64, t546: f64, t555: f64, t8779: f64, t4107: f64, t9288: f64, t10107: f64, t3964: f64, t9285: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t47372, t47381, t47389, t47395, t47417) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2456(t47371, t786, t10115, t1441, t4093, t9292, t1432, t1433, t39497, t10111, t1428, t588);
        let (t47429, t47442, t47444, t47450) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2457(t10022, t2453, t268, t39644, t546, t555, t8779, t1432, t4107, t9288, t10107, t3964, t9285);
    (t47372, t47381, t47389, t47395, t47417, t47429, t47442, t47444, t47450)
}
