//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1236/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1236<F: Float>(t22563: F, t22575: F, t22590: F, t22605: F, t1005: F, t6125: F, t384: F, t386: F, t5679: F, t991: F, t1901: F, t3670: F) -> (F, F, F, F) {
    let t22607 = t22563 + t22575 + t22590 + t22605;
    let t22613 = t1005 * t6125;
    let t22617 = t384 * t386 * t5679 * t991;
    let t22619 = t3670 * t1901;
    (t22607, t22613, t22617, t22619)
}
