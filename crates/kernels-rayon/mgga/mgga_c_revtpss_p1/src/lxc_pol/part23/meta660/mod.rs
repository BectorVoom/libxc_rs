//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta660 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2391;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta660(t2851: f64, t25273: f64, t268: f64, t271: f64, t11852: f64, t159: f64, t907: f64, t9292: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t41296, t41306, t41307, t41329, t41339, t41361) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2391(t2851, t25273, t268, t271, t11852, t159, t907, t9292);
    (t41296, t41306, t41307, t41329, t41339, t41361)
}
