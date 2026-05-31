//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 565/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk565<F: Float>(t134: F, t443: F, t147: F, t135: F, t146: F, t3365: F, t3080: F, t1554: F, t530: F, t161: F, t516: F) -> (F, F, F, F, F, F, F, F) {
    let t3403 = F::cast_from(1.0_f64) / t134 / t443;
    let t3404 = t147 * t3403;
    let t3413 = F::cast_from(0.02962962962962963_f64) * t146 * t3365 * t135;
    let t3414 = F::cast_from(0.11197407407407407_f64) * t3080;
    let t3450 = t1554 * t530;
    let t3451 = t161 * t3450;
    let t3456 = t516 * t516;
    let t3457 = F::cast_from(1.0_f64) / t3456;
    (t3403, t3404, t3413, t3414, t3450, t3451, t3456, t3457)
}
