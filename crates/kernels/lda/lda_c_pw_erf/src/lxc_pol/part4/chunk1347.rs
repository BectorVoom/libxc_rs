//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1347/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1347<F: Float>(t2765: F, t7199: F, t2775: F, t774: F, t7191: F, t102: F, t5548: F, t763: F, t127: F, t14732: F, t14734: F, t1568: F, t1697: F, t1852: F, t411: F, t6121: F, t7116: F, t8862: F, t8865: F, t8867: F, t8869: F, t8871: F, t8873: F, t8877: F) -> (F, F, F, F, F) {
    let t19425 = t2765 * t7199;
    let t19432 = t2775 * t774;
    let t19449 = t2765 * t7191;
    let t19503 = 11.6921 * t102 * t763 * t5548;
    let t19504 = -t8862 + t8865 + t8867 - t8869 + t8871 + t8873 + t8877 - 29.3808 * t127 * t7116 * t1568 + 11.75232 * t127 * t1852 * t5548 + 11.75232 * t127 * t1697 * t6121 * t411 + 28.0 / 27.0 * t14732 + 4.570346666666667 * t14734 + t19503;
    (t19425, t19432, t19449, t19503, t19504)
}
