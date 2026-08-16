//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta630 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2324;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta630(t159: f64, t2698: f64, t1518: f64, t648: f64, t4292: f64, t94: f64, t1353: f64, t1907: f64, t1583: f64, t775: f64, t890: f64, t1014: f64, t65: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t25273, t27123, t27126, t27153, t27375, t27384, t27527) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2324(t159, t2698, t1518, t648, t4292, t94, t1353, t1907, t1583, t775, t890, t1014, t65);
    (t25273, t27123, t27126, t27153, t27375, t27384, t27527)
}
