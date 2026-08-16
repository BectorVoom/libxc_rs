//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 1003/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk1003(t365: f64, t9739: f64, t5047: f64, t5071: f64, t5426: f64, t5439: f64, t5440: f64, t5445: f64, t5448: f64, t9628: f64, t9746: f64, t9753: f64, t9756: f64, t9922: f64, t9925: f64, t9929: f64, t9933: f64, t9936: f64, t9943: f64) -> (f64, f64) {
    let t10854 = t365 * t9739;
    let t10869 = -t5440 + t5445 + t5426 + t5439 + 1.2466946262544771_f64 * t5047 - t5448 + 0.41556487541815906_f64 * t5071 + 12.5_f64 * t9922 - 12.5_f64 * t9925 - 12.5_f64 * t9929 + 18.75_f64 * t9933 - 12.5_f64 * t9936 + 1.2466946262544771_f64 * t9746 + 0.41556487541815906_f64 * t9753 + 1.2466946262544771_f64 * t9756 + 2.4933892525089543_f64 * t9628 - 4.166666666666667_f64 * t9943;
    (t10854, t10869)
}
