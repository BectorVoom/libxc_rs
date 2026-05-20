//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1912/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1912<F: Float>(t4292: F, t648: F, t1907: F, t4144: F, t3829: F, t13514: F, t94: F, t4135: F, t13716: F, t1450: F, t28166: F, t7234: F) -> (F, F, F, F, F, F, F) {
    let t98487 = t648 * t4292;
    let t98496 = t1907 * t4144;
    let t98519 = t1907 * t3829;
    let t98535 = t94 * t13514;
    let t98550 = t1907 * t4135;
    let t98564 = t1450 * t13716;
    let t98579 = t7234 * t28166;
    (t98487, t98496, t98519, t98535, t98550, t98564, t98579)
}
