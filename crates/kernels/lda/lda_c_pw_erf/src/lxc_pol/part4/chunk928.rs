//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 928/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk928<F: Float>(t325: F, t333: F, t912: F, t936: F, t1039: F, t1070: F, t38: F, t36: F, t88: F, t1063: F, t35: F, t1035: F, t1064: F, t3128: F, t3130: F, t905: F, t935: F) -> (F, F, F, F, F, F, F, F) {
    let t8509 = 0.4274 * t325 * t912 * t333 * t936;
    let t8510 = t1070 * t1039;
    let t8512 = t38 * t38;
    let t8516 = 840.0 * t36 / t8512 * t88;
    let t8520 = t35 * t1063;
    let t8521 = t8520 * t88;
    let t8527 = 120.0 * t1064 * t1035;
    let t8528 = t1064 * t1039;
    let t8533 = 3103.50088234237 * t3128 * t935 * t3130 * t905;
    (t8509, t8510, t8516, t8520, t8521, t8527, t8528, t8533)
}
