//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 768/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk768<F: Float>(t3311: F, t3316: F, t3320: F, t3324: F, t3327: F, t3328: F, t3331: F, t3335: F, t5354: F, t5356: F, t5363: F, t5674: F, t5675: F, t6586: F, t6588: F, t6590: F) -> F {
    let t7217 = t5354 - t5356 - t5363 + t5674 + F::new(16.0) / F::new(3.0) * t5675 - t3311 + F::cast_from(0.21642082724729686_f64) * t3316 + F::cast_from(0.011181742741110338_f64) * t3320 + t3324 + t3327 + F::cast_from(0.07214027574909895_f64) * t3328 + t3331 - t3335 - t6586 - t6588 - t6590;
    t7217
}
