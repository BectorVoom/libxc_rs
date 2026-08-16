//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta536 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2341;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2342;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2343;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta536(t17454: f64, t5332: f64, t3720: f64, t1214: f64, t1250: f64, t5346: f64, t16725: f64, t5312: f64, t16729: f64, t1222: f64, t12855: f64, t12910: f64, t13069: f64, t17437: f64, t17438: f64, t17444: f64, t17447: f64, t17448: f64, t17453: f64, t1797: f64, t3631: f64, t3674: f64, t140: f64, t3698: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t17455, t17456, t17459) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2341(t17454, t5332, t3720, t1214, t1250);
        let (t17460, t17461, t17464, t17467, t17470) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2342(t17459, t5346, t3720, t16725, t5312, t16729, t1222, t12855, t12910, t13069, t17437, t17438, t17444, t17447, t17448, t17453, t17456, t1797, t3631, t3674);
        let t17471 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2343(t140, t3698);
    (t17455, t17456, t17459, t17460, t17461, t17464, t17467, t17470, t17471)
}
