//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1006/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1006<F: Float>(t384: F, t398: F, t429: F, t5087: F, t1487: F, t368: F, t879: F, t3101: F, t506: F, t13039: F, t527: F, t1140: F, t5188: F) -> (F, F, F, F, F) {
    let t16916 = t384 * t398 * t429 * t5087;
    let t16921 = t384 * t398 * t368 * t1487 * t879;
    let t16926 = t384 * t398 * t368 * t506 * t3101;
    let t16928 = t13039 * t527;
    let t16930 = t1140 * t5188;
    (t16916, t16921, t16926, t16928, t16930)
}
