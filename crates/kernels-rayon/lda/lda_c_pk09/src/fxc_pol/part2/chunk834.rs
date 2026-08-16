//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 834/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk834(t121: f64, t8493: f64, t633: f64, t8488: f64, t707: f64, t2288: f64, t4668: f64, t823: f64, t3262: f64, t7731: f64, t8338: f64, t890: f64) -> (f64, f64, f64, f64, f64) {
    let t8494 = t121 * t8493;
    let t8497 = t8488 * t633;
    let t8498 = t707 * t8497;
    let t8501 = t2288 * t4668;
    let t8502 = t8501 * t823;
    let t8503 = t121 * t8502;
    let t8506 = t3262 * t7731;
    let t8508 = t890 * t8338;
    (t8494, t8498, t8503, t8506, t8508)
}
