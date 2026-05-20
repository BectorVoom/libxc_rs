//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1473/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1473<F: Float>(t1235: F, t371: F, t6645: F, t676: F, t17307: F, t1803: F, t11262: F, t3711: F, t6618: F, t3609: F, t69692: F, t17416: F, t5381: F) -> (F, F, F, F, F) {
    let t70263 = t1235 * t371 * t676 * t6645;
    let t70267 = t17307 * t1803;
    let t70278 = t3711 * t11262 * t6618;
    let t70319 = t69692 * t3609;
    let t70405 = t5381 * t17416;
    (t70263, t70267, t70278, t70319, t70405)
}
