//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta496 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1801;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1802;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta496(t1401: f64, t26024: f64, t241: f64, t7262: f64, t820: f64, t3920: f64, t7246: f64, t2023: f64, t2453: f64, t3908: f64, t72: f64, t7307: f64, t686: f64, t7284: f64, t1426: f64, t786: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t26025, t26028) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1801(t1401, t26024, t241, t7262, t820);
        let (t26040, t26041, t26043, t26049, t26050, t26051, t26053, t26054) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1802(t3920, t7246, t2023, t2453, t3908, t72, t7307, t686, t7284, t1426, t786);
    (t26025, t26028, t26040, t26041, t26043, t26049, t26050, t26051, t26053, t26054)
}
