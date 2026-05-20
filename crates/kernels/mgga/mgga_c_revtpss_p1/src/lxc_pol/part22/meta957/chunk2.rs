//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3209/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3209<F: Float>(t13312: F, t13392: F, t1486: F, t1927: F, t19680: F, t21686: F, t21695: F, t21698: F, t21699: F, t21702: F, t21727: F, t21768: F, t2291: F, t2312: F, t36: F, t5826: F, t5827: F, t607: F, t60754: F, t60834: F, t60838: F, t627: F, t641: F, t70: F, t85: F) -> F {
    let t60871 = -t21686 * t1927 * t13312 / F::new(6.0) - t60834 * t70 * t85 / F::new(12.0) - t60838 * t70 * t85 / F::new(6.0) - t19680 * t627 * t85 / F::new(6.0) - t21695 * t641 / F::new(6.0) - t36 * t60754 * t70 * t85 / F::new(12.0) - t21698 * t627 * t85 / F::new(6.0) - t21699 * t641 / F::new(6.0) - t607 * t21768 * t85 / F::new(6.0) - t21727 * t641 / F::new(6.0) - t5826 * t2291 * t85 / F::new(12.0) - t21702 * t641 / F::new(6.0) - t5827 * t2312 / F::new(12.0) - t13392 * t1486 * t85 / F::new(6.0);
    t60871
}
