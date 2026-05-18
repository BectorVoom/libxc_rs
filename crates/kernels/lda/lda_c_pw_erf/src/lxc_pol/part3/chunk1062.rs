//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1062/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1062<F: Float>(t3476: F, t4500: F, t12114: F, t4488: F, t10015: F, t5148: F, t739: F, t944: F, t348: F, t3965: F, t5147: F, t5136: F, t945: F) -> (F, F, F, F, F, F) {
    let t12439 = t4500 * t3476;
    let t12442 = F::new(8.0) / F::new(3.0) * t4488 * t12439 * t12114;
    let t12444 = F::new(16.0) / F::new(9.0) * t10015 * t5148;
    let t12445 = t739 * t944;
    let t12446 = t12445 * t348;
    let t12449 = F::new(8.0) / F::new(9.0) * t3965 * t5147 * t12446;
    let t12450 = t5136 * t945;
    (t12442, t12444, t12445, t12446, t12449, t12450)
}
