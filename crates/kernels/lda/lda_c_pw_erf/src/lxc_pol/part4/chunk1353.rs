//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1353/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1353<F: Float>(t156: F, t426: F, t7133: F, t7137: F, t431: F, t5594: F, t7102: F, t102: F, t411: F, t7154: F, t1568: F, t2619: F, t2615: F, t10: F, t1664: F, t19590: F, t2624: F, t2627: F, t5548: F, t7116: F, t7123: F, t767: F) -> (F, F, F, F) {
    let t19593 = t426 * t156 * t7133;
    let t19604 = t426 * t156 * t7137;
    let t19614 = t431 * t7102 * t5594;
    let t19626 = 11.6921 * t102 * t7154 * t411;
    let t19629 = 5.84605 * t102 * t2619 * t1568;
    let t19632 = 17.53815 * t102 * t2615 * t1568;
    let t19633 = 4.0 * t19590 - 2.0 * t19593 - 6.0 * t426 * t10 * t2624 * t1568 + 3.0 * t426 * t10 * t767 * t5548 - t19604 + 3.0 * t426 * t10 * t7123 * t411 + 3.0 / 2.0 * t426 * t10 * t2627 * t1568 - 5.87616 * t19614 - 6.0 * t426 * t10 * t7102 * t1664 + 30.0 * t426 * t10 * t7116 * t1664 + t19626 + t19629 - t19632;
    (t19626, t19629, t19632, t19633)
}
