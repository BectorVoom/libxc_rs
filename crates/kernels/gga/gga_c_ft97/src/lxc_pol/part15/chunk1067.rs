//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1067/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1067<F: Float>(t1969: F, t446: F, t86630: F, t86618: F, t9073: F, t39693: F, t86626: F, t86614: F, t9049: F, t27: F, t526: F, t86868: F, t89: F) -> (F, F, F, F, F) {
    let t87024 = t446 * t1969 * t86630;
    let t87027 = t446 * t9073 * t86618;
    let t87030 = t446 * t39693 * t86626;
    let t87033 = t446 * t9049 * t86614;
    let t87037 = t89 * t27 * t526 * t86868;
    (t87024, t87027, t87030, t87033, t87037)
}
