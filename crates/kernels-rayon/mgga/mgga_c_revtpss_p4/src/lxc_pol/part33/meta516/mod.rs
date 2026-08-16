//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta516 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1853;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta516(t33: f64, t892: f64, t4433: f64, t18875: f64, t25759: f64, t1113: f64, t1544: f64, t4343: f64, t27375: f64, t11064: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t27763, t27764, t27770, t27773, t27777, t27793, t27799) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1853(t33, t892, t4433, t18875, t25759, t1113, t1544, t4343, t27375, t11064);
    (t27763, t27764, t27770, t27773, t27777, t27793, t27799)
}
