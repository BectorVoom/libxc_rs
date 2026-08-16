//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta391 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1839;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1840;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta391(t12772: f64, t3630: f64, t3625: f64, t3372: f64, t5405: f64, t3626: f64, t3368: f64, t3624: f64, t3746: f64, t3618: f64, t828: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t12773, t12774, t12776, t12777, t12780, t12781, t12784) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1839(t12772, t3630, t3625, t3372, t5405, t3626, t3368, t3624, t3746);
        let t12787 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1840(t3618, t828);
    (t12773, t12774, t12776, t12777, t12780, t12781, t12784, t12787)
}
