//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 722/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk722(t2098: f64, t789: f64, t1313: f64, t519: f64, t1976: f64, t806: f64, t4848: f64, t2433: f64, t494: f64, t1326: f64, t1325: f64, t6416: f64, t6421: f64, t6425: f64, t6430: f64, t6435: f64, t6437: f64, t6439: f64, t6441: f64, t6445: f64, t6449: f64, t6451: f64, t6453: f64, t6457: f64, t6459: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t6460 = t789 * t2098;
    let t6461 = t1313 * t6460;
    let t6463 = 8.0_f64 / 45.0_f64 * t519 * t6461;
    let t6464 = t1976 * t806;
    let t6465 = t4848 * t6464;
    let t6467 = 16.0_f64 / 45.0_f64 * t519 * t6465;
    let t6468 = t2433 * t494;
    let t6469 = t1326 * t6468;
    let t6471 = 16.0_f64 / 45.0_f64 * t1325 * t6469;
    let t6472 = t6416 + t6421 + t6425 + t6430 + t6435 - t6437 - t6439 + t6441 - t6445 + t6449 + t6451 + t6453 + t6457 - t6459 - t6463 - t6467 - t6471;
    (t6460, t6461, t6463, t6464, t6465, t6467, t6468, t6469, t6471, t6472)
}
