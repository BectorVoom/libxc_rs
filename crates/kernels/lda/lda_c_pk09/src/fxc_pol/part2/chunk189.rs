//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 189/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk189<F: Float>(t613: F, t617: F, t626: F, t636: F, t653: F, t186: F, t187: F, t183: F, t10: F, t34: F, t39: F, t41: F) -> (F, F, F, F, F, F) {
    let t655 = t613 + t617 + 0.9421211958699838 * t626 + 0.9421211958699838 * t636 - 0.9421211958699838 * t653;
    let t658 = 1.0 / t187 / t186;
    let t659 = t183 * t658;
    let t660 = t34 * t10;
    let t661 = t39 * t41;
    let t662 = t660 * t661;
    (t655, t658, t659, t660, t661, t662)
}
