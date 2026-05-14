//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1029/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1029<F: Float>(t12362: F, t21414: F, t4501: F, t20733: F, t4488: F, t4494: F, t12387: F, t20737: F, t12810: F, t12815: F, t12839: F, t12863: F, t12870: F, t21455: F, t21459: F, t21464: F, t21467: F, t21471: F, t21474: F) -> (F, F, F, F) {
    let t21477 = 16.0 / 9.0 * t12362 * t4501 * t21414;
    let t21480 = 8.0 / 15.0 * t4488 * t4494 * t20733;
    let t21483 = 8.0 / 5.0 * t4488 * t12387 * t20737;
    let t21484 = -t12810 + t12815 + t12839 + t12863 - t12870 - t21455 + t21459 + t21464 - t21467 + t21471 - t21474 - t21477 + t21480 + t21483;
    (t21477, t21480, t21483, t21484)
}
