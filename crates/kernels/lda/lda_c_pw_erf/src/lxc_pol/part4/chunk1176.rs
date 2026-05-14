//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1176/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1176<F: Float>(t12820: F, t12831: F, t12838: F, t12841: F, t12859: F, t12862: F, t12869: F, t12890: F, t3974: F, t4671: F, t6752: F, t4680: F, t11913: F, t4666: F, t811: F, t12143: F, t6756: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t17368 = 32.0 / 45.0 * t12820;
    let t17369 = 32.0 / 135.0 * t12831;
    let t17370 = 32.0 / 405.0 * t12838;
    let t17371 = 32.0 / 135.0 * t12841;
    let t17372 = 16.0 / 135.0 * t12859;
    let t17373 = 64.0 / 405.0 * t12862;
    let t17374 = 32.0 / 243.0 * t12869;
    let t17375 = 32.0 / 135.0 * t12890;
    let t17378 = 32.0 / 9.0 * t3974 * t6752 * t4671;
    let t17381 = 16.0 / 27.0 * t3974 * t6752 * t4680;
    let t17385 = 128.0 / 81.0 * t3974 * t11913 * t811 * t4666;
    let t17387 = 32.0 / 45.0 * t12143 * t6756;
    (t17368, t17369, t17370, t17371, t17372, t17373, t17374, t17375, t17378, t17381, t17385, t17387)
}
