//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 580/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk580<F: Float>(t105: F, t4165: F, t3163: F, t3397: F, t3409: F, t3332: F, t3339: F, t3330: F, t3444: F, t3453: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t4175 = t105 * t4165;
    let t4177 = t4175 * t3163 / F::new(3.0);
    let t4187 = F::cast_from(0.9846956772543541_f64) * t3397;
    let t4190 = F::cast_from(4.431130547644593_f64) * t3409;
    let t4191 = F::cast_from(0.3928367389853144_f64) * t3332;
    let t4192 = F::cast_from(0.06547278983088574_f64) * t3339;
    let t4201 = F::cast_from(0.2946275542389858_f64) * t3330;
    let t4208 = F::cast_from(4.431130547644593_f64) * t3444;
    let t4210 = F::cast_from(11.81634812705225_f64) * t3453;
    let t4231 = F::cast_from(1.0215352034137888_f64) * t3397;
    let t4234 = F::cast_from(4.59690841536205_f64) * t3409;
    let t4235 = F::cast_from(0.4075335835602392_f64) * t3332;
    let t4236 = F::cast_from(0.06792226392670653_f64) * t3339;
    (t4177, t4187, t4190, t4191, t4192, t4201, t4208, t4210, t4231, t4234, t4235, t4236)
}
