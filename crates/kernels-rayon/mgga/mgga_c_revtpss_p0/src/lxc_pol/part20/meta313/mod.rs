//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta313 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1219;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1220;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1221;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta313(t12621: f64, t1280: f64, t3634: f64, t828: f64, t3630: f64, t3625: f64, t3372: f64, t5405: f64, t3626: f64, t3368: f64, t3624: f64, t3746: f64, t3618: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t12769, t12772) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1219(t12621, t1280, t3634, t828);
        let (t12773, t12774, t12776, t12777, t12780, t12781, t12784) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1220(t12772, t3630, t3625, t3372, t5405, t3626, t3368, t3624, t3746);
        let t12787 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1221(t3618, t828);
    (t12769, t12772, t12773, t12774, t12776, t12777, t12780, t12781, t12784, t12787)
}
