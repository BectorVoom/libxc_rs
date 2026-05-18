//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 717/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk717<F: Float>(t9744: F, t9745: F, t446: F, t241: F, t9577: F, t9571: F, t2345: F, t89: F, t2594: F, t9583: F, t2413: F, t713: F) -> (F, F, F, F, F, F, F, F) {
    let t9746 = t9744 * t9745;
    let t9747 = t446 * t9746;
    let t9749 = t241 * t9577;
    let t9750 = t9749 * t9571;
    let t9752 = t89 * t2345 * t9750;
    let t9754 = t2594 * t9583;
    let t9755 = t446 * t9754;
    let t9757 = t2413 * t713;
    (t9746, t9747, t9749, t9750, t9752, t9754, t9755, t9757)
}
