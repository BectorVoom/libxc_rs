//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta635 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2406;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2407;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta635(t2448: f64, t9292: f64, t11036: f64, t2435: f64, t10910: f64, t213: f64, t10994: f64, t2453: f64, t138: f64, t2438: f64, t2771: f64, t2761: f64, t786: f64, t867: f64, t2467: f64, t11043: f64, t10506: f64, t11032: f64, t789: f64, t2458: f64, t2444: f64, t2772: f64, t689: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t41004, t41006, t41008, t41011, t41014, t41017) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2406(t2448, t9292, t11036, t2435, t10910, t213, t10994, t2453, t138, t2438, t2771, t2761, t786, t867);
        let (t41018, t41020, t41021, t41026, t41029, t41032) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2407(t2467, t41017, t11043, t2453, t10506, t11032, t786, t789, t2458, t2761, t2444, t2772, t689);
    (t41004, t41006, t41008, t41011, t41014, t41017, t41018, t41020, t41021, t41026, t41029, t41032)
}
