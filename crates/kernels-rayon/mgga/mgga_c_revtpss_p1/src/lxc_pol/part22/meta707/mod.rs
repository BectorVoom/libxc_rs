//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta707 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2730;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta707(t1868: f64, t4003: f64, t6843: f64, t2723: f64, t6016: f64, t1544: f64, t11660: f64, t1469: f64, t159: f64, t2698: f64, t1518: f64, t648: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t22841, t23037, t23160, t23334, t23898, t25273, t27123) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2730(t1868, t4003, t6843, t2723, t6016, t1544, t11660, t1469, t159, t2698, t1518, t648);
    (t22841, t23037, t23160, t23334, t23898, t25273, t27123)
}
