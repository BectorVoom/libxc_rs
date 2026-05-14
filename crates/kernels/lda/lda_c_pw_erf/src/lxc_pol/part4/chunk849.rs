//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 849/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk849<F: Float>(t1326: F, t6468: F, t1325: F, t6416: F, t6421: F, t6425: F, t6430: F, t6435: F, t6437: F, t6439: F, t6441: F, t6445: F, t6449: F, t6451: F, t6453: F, t6457: F, t6459: F, t6463: F, t6467: F) -> (F, F, F) {
    let t6469 = t1326 * t6468;
    let t6471 = 16.0 / 45.0 * t1325 * t6469;
    let t6472 = t6416 + t6421 + t6425 + t6430 + t6435 - t6437 - t6439 + t6441 - t6445 + t6449 + t6451 + t6453 + t6457 - t6459 - t6463 - t6467 - t6471;
    (t6469, t6471, t6472)
}
