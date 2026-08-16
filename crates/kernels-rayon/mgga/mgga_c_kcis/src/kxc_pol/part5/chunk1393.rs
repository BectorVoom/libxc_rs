//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1393/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1393(t1572: f64, t22979: f64, t12761: f64, t1564: f64, t21311: f64, t21356: f64, t21359: f64, t21362: f64, t21365: f64, t21369: f64, t21372: f64, t21376: f64, t21402: f64, t4326: f64, t6075: f64, t6098: f64, t7444: f64, t7460: f64) -> f64 {
    let t22980 = t22979 * t1572;
    let t22983 = -t21356 + t21359 + t21362 + t21365 - t21369 - t21372 - t21376 - 0.19751789702565206229e-1_f64 * t21311 + t21402 + 2.0_f64 * t6075 * t6098 - 2.0_f64 * t12761 * t7444 + 1.0_f64 * t4326 * t7460 + 1.0_f64 * t1564 * t22980;
    t22983
}
