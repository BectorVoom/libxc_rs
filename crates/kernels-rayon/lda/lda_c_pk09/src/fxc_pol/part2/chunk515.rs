//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 515/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk515(t143: f64, t2983: f64, t569: f64, t933: f64, t17: f64, t24: f64, t580: f64, t68: f64, t1146: f64, t228: f64, t21: f64, t12: f64, t567: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t3032 = t143 * t2983;
    let t3034 = t933 * t569;
    let t3039 = t24 / t580 / t17 * t68;
    let t3040 = t228 * t1146;
    let t3041 = t3040 * t21;
    let t3044 = t12 * t567;
    (t3032, t3034, t3039, t3040, t3041, t3044)
}
