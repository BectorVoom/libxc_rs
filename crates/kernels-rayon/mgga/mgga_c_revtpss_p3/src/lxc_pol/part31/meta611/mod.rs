//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta611 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2053;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta611(t25081: f64, t7897: f64, t2033: f64, t47672: f64, t2: f64, t2411: f64, t198: f64, t206: f64, t7782: f64, t892: f64, t1468: f64, t11064: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t98450, t98495, t98631, t98637, t98646, t98658, t98722) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2053(t25081, t7897, t2033, t47672, t2, t2411, t198, t206, t7782, t892, t1468, t11064);
    (t98450, t98495, t98631, t98637, t98646, t98658, t98722)
}
