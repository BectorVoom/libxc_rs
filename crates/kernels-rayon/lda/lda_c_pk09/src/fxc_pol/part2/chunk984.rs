//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 984/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk984(t1416: f64, t2615: f64, t1397: f64, t2621: f64, t5047: f64, t5071: f64, t5150: f64, t5187: f64, t5191: f64, t5209: f64, t5215: f64, t9628: f64, t9746: f64, t9753: f64, t9756: f64, t9922: f64, t9925: f64, t9929: f64, t9933: f64, t9936: f64, t9943: f64) -> (f64, f64, f64) {
    let t10535 = t2615 * t1416;
    let t10540 = t2621 * t1397;
    let t10555 = -t5191 + t5209 + t5150 + t5187 + 0.505765839233979_f64 * t5047 - t5215 + 0.168588613077993_f64 * t5071 + 4.0_f64 * t9922 - 4.0_f64 * t9925 - 4.0_f64 * t9929 + 6.0_f64 * t9933 - 4.0_f64 * t9936 + 0.505765839233979_f64 * t9746 + 0.168588613077993_f64 * t9753 + 0.505765839233979_f64 * t9756 + 1.011531678467958_f64 * t9628 - 1.3333333333333333_f64 * t9943;
    (t10535, t10540, t10555)
}
