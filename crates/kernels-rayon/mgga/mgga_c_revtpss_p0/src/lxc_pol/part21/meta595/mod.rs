//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta595 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2312;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2313;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2314;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta595(t9163: f64, t99: f64, t107: f64, t9232: f64, t5672: f64, t828: f64, t4363: f64, t13225: f64, t575: f64, t1464: f64, t4153: f64, t1455: f64, t4168: f64, t13250: f64, t571: f64, t2565: f64, t702: f64, t9305: f64, t2576: f64, t2585: f64, t9274: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t36227, t36415, t36776, t36833, t39397, t39399, t39401) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2312(t9163, t99, t107, t9232, t5672, t828, t4363, t13225, t575, t1464, t4153, t1455, t4168);
        let (t39403, t39419) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2313(t13250, t571, t2565, t702, t9305);
        let t39422 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2314(t2576, t2585, t9274);
    (t36227, t36415, t36776, t36833, t39397, t39399, t39401, t39403, t39419, t39422)
}
