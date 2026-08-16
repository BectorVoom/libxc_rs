//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta410 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1352;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta410(t2851: f64, t25273: f64, t268: f64, t271: f64, t11852: f64, t159: f64, t273: f64, t270: f64, t276: f64, t39484: f64, t2922: f64, t275: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t41296, t41306, t41307, t41329, t41339, t41382, t41401, t41499) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1352(t2851, t25273, t268, t271, t11852, t159, t273, t270, t276, t39484, t2922, t275);
    (t41296, t41306, t41307, t41329, t41339, t41382, t41401, t41499)
}
