//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1376/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1376(t1347: f64, t2454: f64, t117: f64, t123: f64, t315: f64, t7228: f64, t10886: f64, t118: f64, t14527: f64, t14529: f64, t14533: f64, t14535: f64, t14539: f64, t14541: f64, t14543: f64, t14545: f64, t14547: f64, t14549: f64, t18059: f64, t18062: f64, t18064: f64, t18066: f64, t18069: f64) -> f64 {
    let t18071 = t2454 * t1347;
    let t18076 = t123 * t315 * t7228 * t117;
    let t18087 = -t10886 - 0.0004954275694490498_f64 * t18059 + 0.06301081444628223_f64 * t18062 + 0.06301081444628223_f64 * t18064 - 0.031505407223141116_f64 * t18066 * t118 - 0.06301081444628223_f64 * t18069 - 0.031505407223141116_f64 * t18071 + 0.1756220988170676_f64 * t14527 + 0.017961351015381915_f64 * t18076 - 0.06301081444628223_f64 * t14529 - 0.06301081444628223_f64 * t14533 - 0.12602162889256446_f64 * t14535 + 0.017961351015381915_f64 * t14539 + 0.1890324433388467_f64 * t14541 - 0.2520432577851289_f64 * t14543 - 0.3780648866776934_f64 * t14545 + 0.06301081444628223_f64 * t14547 + 0.2520432577851289_f64 * t14549;
    t18087
}
