//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta646 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2095;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta646(t104695: f64, t13148: f64, t104707: f64, t1285: f64, t12987: f64, t7623: f64, t5261: f64, t1230: f64, t29082: f64, t29037: f64, t3636: f64, t5326: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t104715, t104721, t104727, t104732, t104739, t104742, t104752) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2095(t104695, t13148, t104707, t1285, t12987, t7623, t5261, t1230, t29082, t29037, t3636, t5326);
    (t104715, t104721, t104727, t104732, t104739, t104742, t104752)
}
