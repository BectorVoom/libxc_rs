//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2312/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2312<F: Float>(t9163: F, t99: F, t107: F, t9232: F, t5672: F, t828: F, t4363: F, t13225: F, t575: F, t1464: F, t4153: F, t1455: F, t4168: F) -> (F, F, F, F, F, F, F) {
    let t36227 = t99 * t9163;
    let t36415 = t107 * t9232;
    let t36776 = t5672 * t828;
    let t36833 = t4363 * t828;
    let t39397 = t13225 * t575;
    let t39399 = t4153 * t1464;
    let t39401 = t1455 * t4168;
    (t36227, t36415, t36776, t36833, t39397, t39399, t39401)
}
