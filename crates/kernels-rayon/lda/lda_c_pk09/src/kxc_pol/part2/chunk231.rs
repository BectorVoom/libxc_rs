//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 231/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk231(t62: f64, t633: f64, t903: f64, t890: f64, t61: f64, t623: f64, t844: f64, t164: f64, t849: f64, t68: f64, t733: f64, t889: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t904 = t62 * t633;
    let t905 = t903 * t904;
    let t906 = t890 * t905;
    let t908 = t61 * t623;
    let t909 = t844 * t908;
    let t910 = t164 * t909;
    let t912 = t61 * t633;
    let t913 = t849 * t912;
    let t914 = t164 * t913;
    let t916 = t733 * t68;
    let t917 = t916 * t889;
    (t904, t905, t906, t908, t909, t910, t912, t913, t914, t916, t917)
}
