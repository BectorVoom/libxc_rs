//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1320/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1320<F: Float>(t13399: F, t13407: F, t17164: F, t17166: F, t17169: F, t17172: F, t17175: F, t17177: F, t17190: F, t17193: F, t9530: F, t9532: F, t9552: F) -> F {
    let t17361 = -F::cast_from(0.0013993827160493828_f64) * t17164 + F::cast_from(0.01847185185185185_f64) * t17166 + F::cast_from(0.002099074074074074_f64) * t17169 + F::cast_from(0.005597530864197531_f64) * t17172 - F::cast_from(0.007556666666666666_f64) * t17175 - F::cast_from(0.007556666666666666_f64) * t17177 - F::cast_from(0.007556666666666666_f64) * t17190 + F::cast_from(0.011335_f64) * t17193 - F::cast_from(0.059613703703703703_f64) * t13399 - F::cast_from(0.003918271604938271_f64) * t13407 + F::cast_from(0.0008396296296296296_f64) * t9530 + F::cast_from(0.000559753086419753_f64) * t9532 - F::cast_from(0.003918271604938271_f64) * t9552;
    t17361
}
