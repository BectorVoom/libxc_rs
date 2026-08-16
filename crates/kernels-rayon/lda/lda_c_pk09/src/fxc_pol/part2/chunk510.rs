//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 510/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk510(t2768: f64, t2863: f64, t2927: f64, t2949: f64, t417: f64, t2138: f64, t2440: f64, t2445: f64, t2696: f64, t2701: f64, t209: f64, t414: f64) -> (f64, f64, f64, f64) {
    let t2951 = t2768 + t2863 + t2927 + t2949;
    let t2952 = t417 * t2951;
    let t2954 = t2138 / 4.0_f64 + t2440 / 4.0_f64 + t2445 / 8.0_f64 + t2696 / 8.0_f64 + t2701 / 8.0_f64 + t2952 / 8.0_f64;
    let t2956 = t209 / 2.0_f64;
    let t2957 = t414 / 4.0_f64;
    (t2951, t2954, t2956, t2957)
}
