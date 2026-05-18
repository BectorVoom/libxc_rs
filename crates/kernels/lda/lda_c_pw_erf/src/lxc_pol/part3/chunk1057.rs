//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1057/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1057<F: Float>(t3518: F, t3892: F, t529: F, t12114: F, t4488: F, t12362: F, t12364: F, t4501: F, t1245: F, t4722: F, t494: F, t739: F, t940: F) -> (F, F, F, F) {
    let t12380 = t3892 * t529 * t3518;
    let t12383 = F::new(32.0) / F::new(27.0) * t4488 * t12380 * t12114;
    let t12386 = F::new(16.0) / F::new(9.0) * t12362 * t4501 * t12364;
    let t12387 = t4722 * t1245;
    let t12389 = t739 * t940 * t494;
    (t12383, t12386, t12387, t12389)
}
