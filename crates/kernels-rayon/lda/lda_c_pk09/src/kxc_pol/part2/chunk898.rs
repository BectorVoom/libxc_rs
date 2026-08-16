//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 898/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk898(t1062: f64, t2393: f64, t721: f64, t2304: f64, t3743: f64, t1106: f64, t8092: f64, t1067: f64, t2394: f64, t2380: f64, t1026: f64, t115: f64, t2341: f64, t4088: f64, t4426: f64, t4494: f64, t4497: f64, t4499: f64, t4504: f64, t4512: f64, t4519: f64, t713: f64, t8977: f64, t9159: f64) -> f64 {
    let t9512 = t2393 * t1062;
    let t9513 = t9512 * t721;
    let t9515 = t2304 * t3743;
    let t9522 = t1106 * t8092;
    let t9526 = t2394 * t1067;
    let t9537 = t2380 * t1067;
    let t9539 = t9513 / 6.0_f64 - t9515 * t4494 / 18.0_f64 - t9515 * t4426 / 18.0_f64 + t9515 * t4088 / 18.0_f64 + t9522 / 6.0_f64 - t8977 * t713 / 6.0_f64 + t9526 / 9.0_f64 - t1026 * t2341 / 6.0_f64 + t115 * t9159 / 6.0_f64 - t4497 / 6.0_f64 - t4499 / 6.0_f64 + t4504 / 6.0_f64 - t4512 / 6.0_f64 + t4519 / 6.0_f64 + t9537 / 9.0_f64;
    t9539
}
