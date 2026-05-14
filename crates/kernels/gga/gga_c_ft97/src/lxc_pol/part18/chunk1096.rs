//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1096/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1096<F: Float>(t172: F, t22637: F, t5592: F, t5587: F, t22632: F, t22747: F, t5598: F, t22572: F, t22577: F, t5569: F, t1716: F, t70: F, t1720: F, t5572: F, t38149: F, t5568: F) -> (F, F, F, F, F, F, F, F) {
    let t92968 = t22637 * t172 * t5592;
    let t92969 = t5587 * t92968;
    let t92975 = t5598 * t22632 * t22747;
    let t92997 = t5569 * t22572 * t22577;
    let t92999 = t1716 * t70;
    let t93003 = t1720 * t70;
    let t93005 = t5569 * t93003 * t5572;
    let t93011 = t38149 * t5568;
    (t92968, t92969, t92975, t92997, t92999, t93003, t93005, t93011)
}
