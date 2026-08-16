//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1550/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1550<F: Float>(t13042: F, t24663: F, t3172: F, t12910: F, t12916: F, t24740: F, t21143: F, t5378: F, t21192: F, t5391: F, t21107: F, t5265: F) -> (F, F, F, F, F) {
    let t82469 = t13042 * t3172 * t24663;
    let t82491 = t12910 * t12916 * t24740;
    let t82534 = t21143 * t5378;
    let t82536 = t5391 * t21192;
    let t82550 = t21107 * t5265;
    (t82469, t82491, t82534, t82536, t82550)
}
