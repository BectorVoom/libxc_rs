//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 972/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk972<F: Float>(t1435: F, t2583: F, t5047: F, t5071: F, t5989: F, t6002: F, t6003: F, t6008: F, t6011: F, t9628: F, t9746: F, t9753: F, t9756: F, t9922: F, t9925: F, t9929: F, t9933: F, t9936: F, t9943: F) -> (F, F) {
    let t10355 = t2583 * t1435;
    let t10369 = -t6003 + t6008 + t5989 + t6002 + F::cast_from(0.2946275542389858_f64) * t5047 - t6011 + F::cast_from(0.0982091847463286_f64) * t5071 + F::cast_from(2.9540870317630623_f64) * t9922 - F::cast_from(2.9540870317630623_f64) * t9925 - F::cast_from(2.9540870317630623_f64) * t9929 + F::cast_from(4.431130547644593_f64) * t9933 - F::cast_from(2.9540870317630623_f64) * t9936 + F::cast_from(0.2946275542389858_f64) * t9746 + F::cast_from(0.0982091847463286_f64) * t9753 + F::cast_from(0.2946275542389858_f64) * t9756 + F::cast_from(0.5892551084779716_f64) * t9628 - F::cast_from(0.9846956772543541_f64) * t9943;
    (t10355, t10369)
}
