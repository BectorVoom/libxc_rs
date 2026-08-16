//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 974/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk974<F: Float>(t1635: F, t4537: F, t1639: F, t20: F, t5794: F, t1926: F, t4196: F, t4199: F, t4546: F, t4207: F, t13714: F, t10567: F, t197: F) -> (F, F, F, F, F, F, F) {
    let t14095 = t4537 * t1635;
    let t14096 = F::cast_from(0.6492624817418906_f64) * t14095;
    let t14098 = t5794 * t20 * t1639;
    let t14099 = F::cast_from(0.03354522822333102_f64) * t14098;
    let t14100 = t1926 * t4196;
    let t14103 = t4546 * t4199;
    let t14105 = t4546 * t4207;
    let t14140 = F::cast_from(0.0016792592592592592_f64) * t13714;
    let t14200 = t10567 * t197;
    (t14096, t14099, t14100, t14103, t14105, t14140, t14200)
}
