//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1030/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1030<F: Float>(t12025: F, t20737: F, t4488: F, t1401: F, t7456: F, t1466: F, t571: F, t593: F, t2163: F, t6205: F, t4763: F, t6993: F, t15926: F, t581: F, t1318: F, t549: F) -> (F, F, F, F, F, F) {
    let t21489 = 8.0 / 3.0 * t4488 * t12025 * t20737;
    let t21490 = t1401 * t7456;
    let t21494 = 4.0 / 15.0 * t571 * t1466 * t21490 * t593;
    let t21496 = 4.0 / 5.0 * t6205 * t2163;
    let t21498 = 4.0 / 5.0 * t4763 * t6993;
    let t21500 = 4.0 / 5.0 * t15926 * t6993;
    let t21501 = t581 * t7456;
    let t21505 = 4.0 / 15.0 * t1318 * t1466 * t21501 * t549;
    (t21489, t21494, t21496, t21498, t21500, t21505)
}
