//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 798/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk798<F: Float>(t2162: F, t3899: F, t571: F, t1381: F, t2161: F, t1466: F, t2167: F, t3787: F, t1325: F, t1278: F, t2166: F, t1440: F) -> (F, F, F, F, F, F, F, F) {
    let t5371 = t3899 * t2162;
    let t5373 = F::cast_from(16.0_f64) / F::cast_from(45.0_f64) * t571 * t5371;
    let t5374 = t2161 * t1381;
    let t5375 = t1466 * t5374;
    let t5378 = t3787 * t2167;
    let t5380 = F::cast_from(16.0_f64) / F::cast_from(45.0_f64) * t1325 * t5378;
    let t5381 = t2166 * t1278;
    let t5382 = t1440 * t5381;
    (t5371, t5373, t5374, t5375, t5378, t5380, t5381, t5382)
}
