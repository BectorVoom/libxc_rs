//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 816/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk816<F: Float>(t188: F, t3007: F, t5633: F, t7683: F, t7687: F, t7689: F, t7692: F, t7694: F, t7698: F, t7700: F, t7701: F, t7702: F, t7703: F, t7704: F, t7707: F, t7708: F) -> F {
    let t7709 = t7683 + t7687 + t7689 + t7692 + t7694 + t7698 + F::new(4.0) * t5633 + t7700 + t7701 - t7702 - t7703 + F::new(4.0) / F::new(3.0) * t7704 * t188 - t7707 - t7708 + t3007;
    t7709
}
