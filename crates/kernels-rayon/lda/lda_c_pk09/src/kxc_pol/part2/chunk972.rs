//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 972/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk972(t1435: f64, t2583: f64, t5047: f64, t5071: f64, t5989: f64, t6002: f64, t6003: f64, t6008: f64, t6011: f64, t9628: f64, t9746: f64, t9753: f64, t9756: f64, t9922: f64, t9925: f64, t9929: f64, t9933: f64, t9936: f64, t9943: f64) -> (f64, f64) {
    let t10355 = t2583 * t1435;
    let t10369 = -t6003 + t6008 + t5989 + t6002 + 0.2946275542389858_f64 * t5047 - t6011 + 0.0982091847463286_f64 * t5071 + 2.9540870317630623_f64 * t9922 - 2.9540870317630623_f64 * t9925 - 2.9540870317630623_f64 * t9929 + 4.431130547644593_f64 * t9933 - 2.9540870317630623_f64 * t9936 + 0.2946275542389858_f64 * t9746 + 0.0982091847463286_f64 * t9753 + 0.2946275542389858_f64 * t9756 + 0.5892551084779716_f64 * t9628 - 0.9846956772543541_f64 * t9943;
    (t10355, t10369)
}
