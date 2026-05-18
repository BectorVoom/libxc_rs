//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 790/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk790<F: Float>(t4645: F, t5260: F, t439: F, t1901: F, t4655: F, t2010: F, t1074: F, t1906: F, t1385: F, t1438: F, t822: F, t1069: F) -> (F, F, F, F, F, F, F, F) {
    let t5261 = t5260 * t4645;
    let t5263 = F::new(8.0) / F::new(81.0) * t439 * t5261;
    let t5264 = t1901 * t4655;
    let t5266 = F::new(4.0) / F::new(27.0) * t2010 * t5264;
    let t5267 = t1906 * t1074;
    let t5268 = t1385 * t5267;
    let t5270 = t439 * t5268 / F::new(45.0);
    let t5271 = t822 * t1438;
    let t5272 = t5271 * t1069;
    (t5261, t5263, t5264, t5266, t5267, t5268, t5270, t5272)
}
