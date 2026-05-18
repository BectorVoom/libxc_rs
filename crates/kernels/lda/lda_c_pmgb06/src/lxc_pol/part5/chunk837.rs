//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 837/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk837<F: Float>(t5370: F, t5372: F, t5376: F, t5379: F, t7750: F, t7752: F, t7754: F, t7756: F, t7758: F, t7759: F, t7760: F, t7761: F, t7762: F, t7763: F, t7765: F, t7766: F) -> F {
    let t7983 = -t7750 - t7752 - t7754 - t7756 + F::new(4.0) / F::new(3.0) * t5370 - F::new(2.0) / F::new(9.0) * t5372 - t7758 - t7759 - t7760 - t7761 - t7762 - t7763 + t7765 + t7766 + t5376 + F::new(0.36466666666666664) * t5379;
    t7983
}
