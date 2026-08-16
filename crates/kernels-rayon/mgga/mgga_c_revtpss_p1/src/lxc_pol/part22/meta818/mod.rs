//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta818 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2929;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2930;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta818(t14085: f64, t2435: f64, t14104: f64, t47520: f64, t10069: f64, t13731: f64, t137: f64, t14103: f64, t47480: f64, t9675: f64, t14099: f64, t2453: f64, t9676: f64, t14109: f64, t9680: f64, t9685: f64, t5603: f64, t9692: f64, t1904: f64, t689: f64, t9634: f64, t1364: f64, t14067: f64, t786: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t47834, t47837, t47839, t47844, t47856) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2929(t14085, t2435, t14104, t47520, t10069, t13731, t137, t14103, t47480, t9675, t14099, t2453);
        let (t47857, t47860, t47863, t47873, t47876) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2930(t47856, t9676, t14109, t9680, t9685, t5603, t9692, t1904, t689, t9634, t1364, t14067, t786);
    (t47834, t47837, t47839, t47844, t47856, t47857, t47860, t47863, t47873, t47876)
}
