//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 592/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk592<F: Float>(t205: F, t4594: F, t148: F, t733: F, t83: F, t142: F, t3163: F, t3498: F, t810: F, t4280: F, t89: F, t170: F) -> (F, F, F, F, F) {
    let t4595 = t205 * t4594;
    let t4603 = t148 * t148;
    let t4604 = F::new(1.0) / t4603;
    let t4609 = t83 * t733;
    let t4610 = t4609 * t142;
    let t4612 = F::cast_from(38.978347549160304_f64) * t4610 * t3163;
    let t4614 = F::cast_from(25.985565032773536_f64) * t810 * t3498;
    let t4621 = t89 * t4280;
    let t4623 = F::cast_from(0.04572295947761066_f64) * t4621 * t170;
    (t4595, t4604, t4612, t4614, t4623)
}
