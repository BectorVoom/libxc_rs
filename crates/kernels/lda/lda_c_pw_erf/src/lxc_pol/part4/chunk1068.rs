//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1068/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1068<F: Float>(t1449: F, t519: F, t6938: F, t3794: F, t6693: F, t1325: F, t2098: F, t4956: F, t4957: F, t4804: F, t1458: F, t1460: F, t2824: F, t35: F, t6689: F, t12536: F, t799: F) -> (F, F, F, F, F, F, F, F) {
    let t15542 = t519 * t1449 * t6938;
    let t15543 = 16.0 / 135.0 * t15542;
    let t15545 = 16.0 / 15.0 * t3794 * t6693;
    let t15549 = 16.0 / 15.0 * t1325 * t4956 * t4957 * t2098;
    let t15551 = 16.0 / 15.0 * t4804 * t6693;
    let t15556 = 32.0 / 27.0 * t519 * t2824 * t1458 * t1460 * t35;
    let t15557 = t4804 * t6689;
    let t15558 = 32.0 / 45.0 * t15557;
    let t15559 = t3794 * t6689;
    let t15560 = 32.0 / 45.0 * t15559;
    let t15562 = 8.0 / 45.0 * t12536 * t799;
    (t15543, t15545, t15549, t15551, t15556, t15558, t15560, t15562)
}
