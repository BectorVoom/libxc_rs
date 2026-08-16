//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 592/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk592(t205: f64, t4594: f64, t148: f64, t733: f64, t83: f64, t142: f64, t3163: f64, t3498: f64, t810: f64, t4280: f64, t89: f64, t170: f64) -> (f64, f64, f64, f64, f64) {
    let t4595 = t205 * t4594;
    let t4603 = t148 * t148;
    let t4604 = 1.0_f64 / t4603;
    let t4609 = t83 * t733;
    let t4610 = t4609 * t142;
    let t4612 = 38.978347549160304_f64 * t4610 * t3163;
    let t4614 = 25.985565032773536_f64 * t810 * t3498;
    let t4621 = t89 * t4280;
    let t4623 = 0.04572295947761066_f64 * t4621 * t170;
    (t4595, t4604, t4612, t4614, t4623)
}
