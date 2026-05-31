//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 722/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk722<F: Float>(t2098: F, t789: F, t1313: F, t519: F, t1976: F, t806: F, t4848: F, t2433: F, t494: F, t1326: F, t1325: F, t6416: F, t6421: F, t6425: F, t6430: F, t6435: F, t6437: F, t6439: F, t6441: F, t6445: F, t6449: F, t6451: F, t6453: F, t6457: F, t6459: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t6460 = t789 * t2098;
    let t6461 = t1313 * t6460;
    let t6463 = F::cast_from(8.0_f64) / F::cast_from(45.0_f64) * t519 * t6461;
    let t6464 = t1976 * t806;
    let t6465 = t4848 * t6464;
    let t6467 = F::cast_from(16.0_f64) / F::cast_from(45.0_f64) * t519 * t6465;
    let t6468 = t2433 * t494;
    let t6469 = t1326 * t6468;
    let t6471 = F::cast_from(16.0_f64) / F::cast_from(45.0_f64) * t1325 * t6469;
    let t6472 = t6416 + t6421 + t6425 + t6430 + t6435 - t6437 - t6439 + t6441 - t6445 + t6449 + t6451 + t6453 + t6457 - t6459 - t6463 - t6467 - t6471;
    (t6460, t6461, t6463, t6464, t6465, t6467, t6468, t6469, t6471, t6472)
}
