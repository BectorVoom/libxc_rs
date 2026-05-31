//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 984/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk984<F: Float>(t1416: F, t2615: F, t1397: F, t2621: F, t5047: F, t5071: F, t5150: F, t5187: F, t5191: F, t5209: F, t5215: F, t9628: F, t9746: F, t9753: F, t9756: F, t9922: F, t9925: F, t9929: F, t9933: F, t9936: F, t9943: F) -> (F, F, F) {
    let t10535 = t2615 * t1416;
    let t10540 = t2621 * t1397;
    let t10555 = -t5191 + t5209 + t5150 + t5187 + F::cast_from(0.505765839233979_f64) * t5047 - t5215 + F::cast_from(0.168588613077993_f64) * t5071 + F::cast_from(4.0_f64) * t9922 - F::cast_from(4.0_f64) * t9925 - F::cast_from(4.0_f64) * t9929 + F::cast_from(6.0_f64) * t9933 - F::cast_from(4.0_f64) * t9936 + F::cast_from(0.505765839233979_f64) * t9746 + F::cast_from(0.168588613077993_f64) * t9753 + F::cast_from(0.505765839233979_f64) * t9756 + F::cast_from(1.011531678467958_f64) * t9628 - F::cast_from(1.3333333333333333_f64) * t9943;
    (t10535, t10540, t10555)
}
