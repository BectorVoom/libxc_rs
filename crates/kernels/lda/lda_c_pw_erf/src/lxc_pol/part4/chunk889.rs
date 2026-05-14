//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 889/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk889<F: Float>(t102: F, t1832: F, t763: F, t2619: F, t411: F, t2594: F, t3296: F, t436: F, t6121: F, t120: F, t2624: F, t767: F, t2627: F, t156: F, t2615: F, t426: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t7112 = 11.6921 * t102 * t763 * t1832;
    let t7115 = 5.84605 * t102 * t2619 * t411;
    let t7116 = t3296 * t2594;
    let t7123 = t436 * t6121;
    let t7126 = t120 * t6121;
    let t7128 = 2.923025 * t102 * t7126;
    let t7129 = t2624 * t411;
    let t7133 = t767 * t1832;
    let t7137 = t2627 * t411;
    let t7142 = t156 * t2615;
    let t7143 = t426 * t7142;
    (t7112, t7115, t7116, t7123, t7126, t7128, t7129, t7133, t7137, t7142, t7143)
}
