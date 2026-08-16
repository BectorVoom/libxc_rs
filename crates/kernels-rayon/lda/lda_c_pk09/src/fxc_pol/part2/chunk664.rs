//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 664/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk664(t1447: f64, t305: f64, t5039: f64, t5045: f64, t5068: f64, t5161: f64, t5190: f64, t5208: f64, t5212: f64, t1290: f64, t5081: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t6054 = t1447 * t1447;
    let t6055 = 1.0_f64 / t6054;
    let t6056 = t6055 * t305;
    let t6060 = 1.5625_f64 * t5039;
    let t6062 = 1.0416666666666667_f64 * t5045;
    let t6068 = 0.3472222222222222_f64 * t5068;
    let t6078 = 0.64_f64 * t5039;
    let t6082 = 4.277978922036907_f64 * t5161;
    let t6091 = 0.4266666666666667_f64 * t5045;
    let t6092 = 0.35649824350307563_f64 * t5190;
    let t6097 = 3.2084841915276807_f64 * t5208;
    let t6098 = 3.2084841915276807_f64 * t5212;
    let t6100 = 0.14222222222222222_f64 * t5068;
    let t6107 = t1290 * t5081;
    (t6056, t6060, t6062, t6068, t6078, t6082, t6091, t6092, t6097, t6098, t6100, t6107)
}
