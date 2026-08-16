//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta351 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1279;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1280;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1281;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta351(t4144: f64, t9593: f64, t159: f64, t2698: f64, t4135: f64, t4147: f64, t26: f64, t65: f64, t9163: f64, t99: f64, t107: f64, t9232: f64, t13225: f64, t575: f64, t1464: f64, t4153: f64, t1455: f64, t4168: f64, t13250: f64, t571: f64, t2565: f64, t702: f64, t9305: f64, t2576: f64, t2585: f64, t9274: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t25177, t25273, t25802, t33127, t36227, t36415) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1279(t4144, t9593, t159, t2698, t4135, t4147, t26, t65, t9163, t99, t107, t9232);
        let (t39397, t39399, t39401, t39403, t39419) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1280(t13225, t575, t1464, t4153, t1455, t4168, t13250, t571, t2565, t702, t9305);
        let t39422 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1281(t2576, t2585, t9274);
    (t25177, t25273, t25802, t33127, t36227, t36415, t39397, t39399, t39401, t39403, t39419, t39422)
}
