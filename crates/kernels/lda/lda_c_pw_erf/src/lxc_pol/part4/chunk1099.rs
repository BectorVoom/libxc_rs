//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1099/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1099<F: Float>(t515: F, t6631: F, t12044: F, t12046: F, t12050: F, t12052: F, t1524: F, t2528: F, t2076: F, t4571: F, t12765: F, t1325: F, t1392: F, t6431: F, t34: F, t806: F) -> (F, F, F, F, F, F, F, F, F) {
    let t16016 = t6631 * t515;
    let t16017 = 8.0 / 45.0 * t16016;
    let t16018 = 8.0 / 45.0 * t12044;
    let t16019 = 32.0 / 135.0 * t12046;
    let t16020 = 16.0 / 135.0 * t12050;
    let t16021 = 16.0 / 45.0 * t12052;
    let t16023 = 4.0 / 15.0 * t1524 * t2528;
    let t16024 = t2076 * t4571;
    let t16025 = 16.0 / 135.0 * t16024;
    let t16029 = 16.0 / 5.0 * t1325 * t12765 * t6431 * t1392;
    let t16031 = t34 * t806;
    (t16017, t16018, t16019, t16020, t16021, t16023, t16025, t16029, t16031)
}
