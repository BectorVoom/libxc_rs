//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 937/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk937<F: Float>(t3518: F, t3892: F, t529: F, t12114: F, t4488: F, t12362: F, t12364: F, t4501: F, t1245: F, t4722: F, t494: F, t739: F, t940: F, t3965: F, t11857: F, t3412: F, t4483: F) -> (F, F, F, F, F, F) {
    let t12380 = t3892 * t529 * t3518;
    let t12383 = 32.0 / 27.0 * t4488 * t12380 * t12114;
    let t12386 = 16.0 / 9.0 * t12362 * t4501 * t12364;
    let t12387 = t4722 * t1245;
    let t12389 = t739 * t940 * t494;
    let t12392 = 16.0 / 5.0 * t3965 * t12387 * t12389;
    let t12395 = 8.0 / 5.0 * t4488 * t12387 * t11857;
    let t12398 = 8.0 / 15.0 * t4488 * t4483 * t3412;
    (t12383, t12386, t12389, t12392, t12395, t12398)
}
