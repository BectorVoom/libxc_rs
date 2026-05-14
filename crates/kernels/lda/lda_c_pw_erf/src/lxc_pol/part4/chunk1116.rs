//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1116/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1116<F: Float>(t1351: F, t6005: F, t352: F, t11: F, t1349: F, t16100: F, t16105: F, t3633: F, t325: F, t6662: F, t6659: F, t1333: F, t557: F, t6360: F, t954: F, t4606: F, t6646: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t16305 = t1351 * t6005;
    let t16306 = t16305 * t352;
    let t16308 = t11 * t1349 * t16306;
    let t16311 = t11 * t1349 * t16100;
    let t16314 = t11 * t3633 * t16105;
    let t16325 = t325 * t6662;
    let t16327 = t325 * t6659;
    let t16329 = t1333 * t6005;
    let t16330 = t16329 * t352;
    let t16332 = t11 * t557 * t16330;
    let t16334 = t6360 * t954;
    let t16336 = t11 * t557 * t16334;
    let t16338 = t4606 * t6646;
    (t16306, t16308, t16311, t16314, t16325, t16327, t16330, t16332, t16334, t16336, t16338)
}
