//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 529/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk529(t179: f64, t3233: f64, t133: f64, t3086: f64, t131: f64, t735: f64, t567: f64, t741: f64, t743: f64, t205: f64, t568: f64, t727: f64, t728: f64) -> (f64, f64, f64, f64, f64) {
    let t3234 = t179 * t3233;
    let t3236 = t133 * t3086;
    let t3237 = t131 * t3236;
    let t3239 = 2.3693919160612835_f64 * t735 * t3237;
    let t3241 = t567 * t741;
    let t3242 = t3241 * t743;
    let t3243 = t205 * t3242;
    let t3246 = t727 * t568 * t728;
    (t3234, t3239, t3241, t3243, t3246)
}
