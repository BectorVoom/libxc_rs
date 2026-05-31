//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 204/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk204<F: Float>(t543: F, t545: F, t184: F, t187: F, t188: F, t434: F, t438: F, t448: F, t462: F, t481: F, t488: F, t492: F, t502: F, t515: F, t533: F, t534: F, t542: F) -> (F, F, F) {
    let t547 = F::cast_from(0.10821041362364843_f64) * t543 * t545;
    let t549 = F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t184 * t187;
    let t550 = t434 + t438 + t448 + t462 - t481 + t488 + t492 + t502 + t515 - t533 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t534 * t188 + t542 + t547 + t549;
    (t547, t549, t550)
}
