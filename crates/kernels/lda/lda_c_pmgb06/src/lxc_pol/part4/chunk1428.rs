//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1428/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1428<F: Float>(t13444: F, t13447: F, t13450: F, t17561: F, t17564: F, t17571: F, t17575: F, t17576: F, t17578: F, t17583: F, t17584: F, t17585: F, t17587: F, t17588: F, t17589: F) -> F {
    let t18314 = t17561 - t17564 + t17571 - t17575 + t17576 - t17578 - t17583 + t17584 + F::new(4.0) / F::new(3.0) * t13444 + F::new(2.0) / F::new(3.0) * t13447 + F::cast_from(0.36466666666666664_f64) * t13450 + t17585 + t17587 + t17588 + t17589;
    t18314
}
