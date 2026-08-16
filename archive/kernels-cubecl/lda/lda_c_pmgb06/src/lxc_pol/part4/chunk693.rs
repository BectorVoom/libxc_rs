//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 693/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk693<F: Float>(t1698: F, t591: F, t1701: F, t4111: F, t208: F, t315: F, t586: F, t584: F, t1727: F, t607: F, t1710: F, t604: F) -> (F, F, F, F, F, F) {
    let t4115 = F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t1698 * t591;
    let t4117 = F::cast_from(2e-21_f64) * t1701 * t4111;
    let t4119 = t586 * t315 * t208;
    let t4121 = F::cast_from(0.013506172839506173_f64) * t584 * t4119;
    let t4146 = t1727 * t607;
    let t4148 = t604 * t1710;
    (t4115, t4117, t4119, t4121, t4146, t4148)
}
