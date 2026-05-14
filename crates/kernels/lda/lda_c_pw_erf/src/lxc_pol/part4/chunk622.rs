//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 622/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk622<F: Float>(t133: F, t3227: F, t3230: F, t3219: F, t153: F, t274: F, t2869: F, t1089: F, t474: F, t1125: F, t678: F, t1298: F, t1386: F, t1393: F, t514: F, t185: F) -> (F, F, F, F, F, F, F, F, F) {
    let t3349 = t133 * t3227;
    let t3351 = t133 * t3230;
    let t3361 = t133 * t3219;
    let t3373 = 4.429070076315393 * t153 * t2869 * t274;
    let t3375 = t153 * t474 * t1089;
    let t3378 = t153 * t1125 * t678;
    let t3380 = t1298 * t1386;
    let t3384 = t514 * t1393;
    let t3385 = t185 * t3384;
    (t3349, t3351, t3361, t3373, t3375, t3378, t3380, t3384, t3385)
}
