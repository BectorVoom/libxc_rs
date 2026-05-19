//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 708/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk708<F: Float>(t2164: F, t395: F, t4252: F, t4254: F, t4257: F, t4261: F, t4264: F, t4267: F, t4283: F, t4284: F, t4286: F, t4472: F, t4575: F, t81: F) -> F {
    let t4579 = F::cast_from(0.2133002709687175_f64) * t395 * t2164;
    let t4580 = F::cast_from(0.31995040645307626_f64) * t4472 - F::cast_from(0.10665013548435875_f64) * t4286 + F::cast_from(0.6399008129061525_f64) * t4284 + F::cast_from(0.053059442957798957_f64) * t4261 + F::cast_from(0.10611888591559791_f64) * t4264 + F::cast_from(0.053059442957798957_f64) * t4267 - F::cast_from(0.28298369577492777_f64) * t4254 - F::cast_from(0.28298369577492777_f64) * t4257 - t4283 + t4252 + F::cast_from(0.05332506774217938_f64) * t81 * t4575 - t4579;
    t4580
}
