//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1085/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1085<F: Float>(t92353: F, t92356: F, t420: F, t5571: F, t415: F, t5576: F, t22548: F, t25: F, t408: F, t1608: F, t22626: F, t1691: F, t35: F, t22632: F, t22806: F, t5611: F) -> (F, F, F, F, F, F, F) {
    let t92357 = t92353 * t92356;
    let t92358 = t420 * t5571;
    let t92370 = t5576 * t415;
    let t92371 = t22548 * t92370;
    let t92377 = t408 * t25;
    let t92379 = t1608 * t92377 * t22626;
    let t92380 = t35 * t1691;
    let t92385 = t22632 * t22806;
    let t92386 = t5611 * t92385;
    (t92357, t92358, t92371, t92379, t92380, t92385, t92386)
}
