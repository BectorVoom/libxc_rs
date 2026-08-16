//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 80/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk80(t215: f64, t8: f64, t18: f64, t19: f64, t9: f64) -> (f64, f64, f64) {
    let t216 = t215 * t8;
    let t217 = t18 * t18;
    let t221 = f64::exp(-0.1173961225190475_f64 * t19);
    let t225 = 0.41081146652128_f64 + 0.14983581422587874_f64 * t216 * t217 + 0.01928080210487025_f64 * t221 * t9 * t18;
    (t217, t221, t225)
}
