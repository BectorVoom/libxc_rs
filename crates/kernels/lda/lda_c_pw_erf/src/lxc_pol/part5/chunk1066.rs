//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1066/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1066<F: Float>(t1325: F, t1440: F, t15595: F, t784: F, t518: F, t7675: F, t577: F, t4753: F, t7597: F, t3416: F, t1318: F, t1466: F, t17759: F, t833: F, t2065: F, t6991: F) -> (F, F, F, F, F, F) {
    let t22189 = 4.0 / 5.0 * t1325 * t1440 * t15595 * t784;
    let t22190 = t7675 * t518;
    let t22192 = 4.0 / 45.0 * t22190 * t577;
    let t22194 = 4.0 / 5.0 * t4753 * t7597;
    let t22196 = 4.0 / 5.0 * t3416 * t7597;
    let t22200 = 4.0 / 5.0 * t1318 * t1466 * t17759 * t833;
    let t22204 = 4.0 / 5.0 * t1318 * t1466 * t6991 * t2065;
    (t22189, t22192, t22194, t22196, t22200, t22204)
}
