//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 653/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk653<F: Float>(t3783: F, t798: F, t519: F, t3762: F, t825: F, t571: F, t2192: F, t3899: F, t1318: F, t2162: F, t2167: F, t3787: F) -> (F, F, F, F, F, F, F, F, F) {
    let t5339 = t3783 * t798;
    let t5340 = t519 * t5339;
    let t5342 = t3762 * t825;
    let t5343 = t571 * t5342;
    let t5363 = t3899 * t2192;
    let t5365 = F::cast_from(16.0_f64) / F::cast_from(45.0_f64) * t1318 * t5363;
    let t5371 = t3899 * t2162;
    let t5373 = F::cast_from(16.0_f64) / F::cast_from(45.0_f64) * t571 * t5371;
    let t5378 = t3787 * t2167;
    (t5339, t5340, t5342, t5343, t5363, t5365, t5371, t5373, t5378)
}
