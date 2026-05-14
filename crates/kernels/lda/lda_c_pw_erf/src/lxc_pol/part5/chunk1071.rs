//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1071/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1071<F: Float>(t1991: F, t22281: F, t519: F, t2429: F, t34: F, t4829: F, t1318: F, t1319: F, t549: F, t7422: F, t1325: F, t4956: F, t6944: F, t10557: F, t7624: F, t1449: F, t7620: F) -> (F, F, F, F, F, F, F) {
    let t22284 = 16.0 / 3.0 * t519 * t1991 * t22281;
    let t22285 = t2429 * t34;
    let t22288 = 16.0 / 5.0 * t519 * t4829 * t22285;
    let t22292 = 16.0 / 15.0 * t1318 * t1319 * t7422 * t549;
    let t22296 = 8.0 / 5.0 * t1325 * t4956 * t6944 * t34;
    let t22298 = t519 * t10557 * t7624;
    let t22299 = 64.0 / 243.0 * t22298;
    let t22301 = t519 * t1449 * t7620;
    (t22284, t22285, t22288, t22292, t22296, t22299, t22301)
}
