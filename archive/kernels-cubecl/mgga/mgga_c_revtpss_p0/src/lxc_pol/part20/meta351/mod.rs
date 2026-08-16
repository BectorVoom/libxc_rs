//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta351 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1279;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1280;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1281;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta351<F: Float>(t4144: F, t9593: F, t159: F, t2698: F, t4135: F, t4147: F, t26: F, t65: F, t9163: F, t99: F, t107: F, t9232: F, t13225: F, t575: F, t1464: F, t4153: F, t1455: F, t4168: F, t13250: F, t571: F, t2565: F, t702: F, t9305: F, t2576: F, t2585: F, t9274: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t25177, t25273, t25802, t33127, t36227, t36415) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1279::<F>(t4144, t9593, t159, t2698, t4135, t4147, t26, t65, t9163, t99, t107, t9232);
        let (t39397, t39399, t39401, t39403, t39419) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1280::<F>(t13225, t575, t1464, t4153, t1455, t4168, t13250, t571, t2565, t702, t9305);
        let t39422 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1281::<F>(t2576, t2585, t9274);
    (t25177, t25273, t25802, t33127, t36227, t36415, t39397, t39399, t39401, t39403, t39419, t39422)
}
