//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 882/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk882(t1106: f64, t9049: f64, t2354: f64, t4517: f64, t3148: f64, t7795: f64, t7797: f64, t7799: f64, t7801: f64, t7805: f64, t7809: f64, t7811: f64, t7814: f64, t7817: f64, t7834: f64, t7838: f64, t7842: f64, t7846: f64) -> (f64, f64, f64) {
    let t9267 = t1106 * t9049;
    let t9275 = t2354 * t4517;
    let t9276 = t9275 * t3148;
    let t9291 = -5.908174063526125_f64 * t7795 + 5.908174063526125_f64 * t7797 + 5.908174063526125_f64 * t7799 - 0.1964183694926572_f64 * t7801 - 0.2946275542389858_f64 * t7805 - 0.2946275542389858_f64 * t7809 - 0.2946275542389858_f64 * t7811 - 0.2946275542389858_f64 * t7814 - 0.2946275542389858_f64 * t7817 - 0.2946275542389858_f64 * t7834 - 4.431130547644593_f64 * t7838 + 4.431130547644593_f64 * t7842 + 4.431130547644593_f64 * t7846;
    (t9267, t9276, t9291)
}
