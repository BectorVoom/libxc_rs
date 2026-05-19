//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1376/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1376<F: Float>(t1347: F, t2454: F, t117: F, t123: F, t315: F, t7228: F, t10886: F, t118: F, t14527: F, t14529: F, t14533: F, t14535: F, t14539: F, t14541: F, t14543: F, t14545: F, t14547: F, t14549: F, t18059: F, t18062: F, t18064: F, t18066: F, t18069: F) -> F {
    let t18071 = t2454 * t1347;
    let t18076 = t123 * t315 * t7228 * t117;
    let t18087 = -t10886 - F::cast_from(0.0004954275694490498_f64) * t18059 + F::cast_from(0.06301081444628223_f64) * t18062 + F::cast_from(0.06301081444628223_f64) * t18064 - F::cast_from(0.031505407223141116_f64) * t18066 * t118 - F::cast_from(0.06301081444628223_f64) * t18069 - F::cast_from(0.031505407223141116_f64) * t18071 + F::cast_from(0.1756220988170676_f64) * t14527 + F::cast_from(0.017961351015381915_f64) * t18076 - F::cast_from(0.06301081444628223_f64) * t14529 - F::cast_from(0.06301081444628223_f64) * t14533 - F::cast_from(0.12602162889256446_f64) * t14535 + F::cast_from(0.017961351015381915_f64) * t14539 + F::cast_from(0.1890324433388467_f64) * t14541 - F::cast_from(0.2520432577851289_f64) * t14543 - F::cast_from(0.3780648866776934_f64) * t14545 + F::cast_from(0.06301081444628223_f64) * t14547 + F::cast_from(0.2520432577851289_f64) * t14549;
    t18087
}
