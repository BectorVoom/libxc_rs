//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1022/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1022<F: Float>(t2101: F, t2563: F, t161: F, t489: F, t7442: F, t132: F, t137: F, t2064: F, t6734: F, t6610: F, t831: F, t1450: F, t493: F, t7670: F) -> (F, F, F, F, F) {
    let t19224 = t2563 * t2101 / F::cast_from(10.0_f64);
    let t19226 = t161 * t489 * t7442;
    let t19227 = t19226 / F::cast_from(15.0_f64);
    let t19231 = t132 * t137 * t6734 * t2064 / F::cast_from(10.0_f64);
    let t19232 = t831 * t6610;
    let t19233 = F::cast_from(2.0_f64) / F::cast_from(15.0_f64) * t19232;
    let t19236 = F::cast_from(2.0_f64) / F::cast_from(15.0_f64) * t493 * t1450 * t7670;
    (t19224, t19227, t19231, t19233, t19236)
}
