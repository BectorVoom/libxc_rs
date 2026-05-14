//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 320/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk320<F: Float>(t1084: F, t1085: F, t1001: F, t1061: F, t1066: F, t1069: F, t1072: F, t1075: F, t1079: F, t1083: F, t910: F, t938: F, t997: F, t1058: F) -> (F, F) {
    let t1086 = t1084 * t1085;
    let t1087 = 0.010843580882781523 * t1086;
    let t1088 = -t1061 + t1066 + t1069 - t1072 - t997 + t938 + t910 - t1001 - t1075 + t1079 + t1083 + t1087;
    let t1089 = t1058 + t1088;
    (t1087, t1089)
}
