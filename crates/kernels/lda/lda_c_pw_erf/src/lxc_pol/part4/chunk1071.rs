//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1071/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1071<F: Float>(t1450: F, t6198: F, t11900: F, t1390: F, t6566: F, t1440: F, t519: F, t542: F, t529: F, t1325: F, t494: F, t1472: F, t6195: F, t12299: F, t2168: F, t12136: F, t4480: F) -> (F, F, F, F, F, F, F) {
    let t15587 = t6198 * t1450;
    let t15588 = 16.0 / 135.0 * t15587;
    let t15589 = 256.0 / 135.0 * t11900;
    let t15590 = t1390 * t6566;
    let t15594 = 8.0 / 15.0 * t519 * t1440 * t15590 * t542;
    let t15595 = t529 * t6566;
    let t15599 = 8.0 / 15.0 * t1325 * t1440 * t15595 * t494;
    let t15601 = 8.0 / 15.0 * t1472 * t6195;
    let t15603 = 16.0 / 15.0 * t12299 * t2168;
    let t15605 = 32.0 / 45.0 * t12136 * t4480;
    (t15588, t15589, t15594, t15599, t15601, t15603, t15605)
}
