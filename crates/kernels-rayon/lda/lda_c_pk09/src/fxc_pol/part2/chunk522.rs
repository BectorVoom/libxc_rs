//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 522/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk522(t3147: f64, t3148: f64, t61: f64, t902: f64, t149: f64, t733: f64, t142: f64, t609: f64) -> (f64, f64, f64, f64) {
    let t3149 = t3147 * t3148;
    let t3153 = t902 * t61;
    let t3159 = t149 * t733;
    let t3160 = t3159 * t142;
    let t3161 = t609 * t609;
    (t3149, t3153, t3160, t3161)
}
