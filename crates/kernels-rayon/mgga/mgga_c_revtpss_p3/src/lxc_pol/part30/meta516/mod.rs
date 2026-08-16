//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta516 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1915;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1916;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta516(t1544: f64, t1583: f64, t18875: f64, t1940: f64, t1963: f64, t198: f64, t207: f64, t2403: f64, t25440: f64, t25445: f64, t27363: f64, t27368: f64, t27375: f64, t27384: f64, t4343: f64, t4433: f64, t4537: f64, t4541: f64, t7087: f64, t7091: f64, t775: f64, t7783: f64, t890: f64, t892: f64, t33: f64, t25759: f64, t1113: f64, t11064: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let t27754 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1915(t1544, t1583, t18875, t1940, t1963, t198, t207, t2403, t25440, t25445, t27363, t27368, t27375, t27384, t4343, t4433, t4537, t4541, t7087, t7091, t775, t7783, t890, t892);
        let (t27763, t27764, t27770, t27773, t27777, t27793, t27799) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1916(t33, t892, t4433, t18875, t25759, t1113, t1544, t4343, t27375, t11064);
    (t27754, t27763, t27764, t27770, t27773, t27777, t27793, t27799)
}
