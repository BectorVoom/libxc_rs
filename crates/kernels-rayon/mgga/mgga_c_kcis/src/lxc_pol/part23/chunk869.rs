//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 869/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk869(t2645: f64, t5488: f64, t3766: f64, t1961: f64, t3754: f64, t2642: f64, t11374: f64, t1409: f64, t3786: f64, t1319: f64, t1419: f64, t16073: f64, t5439: f64) -> (f64, f64, f64, f64) {
    let t16378 = t5488 * t2645;
    let t16379 = t3766 * t16378;
    let t16382 = t1961 * t3754;
    let t16383 = t16382 * t2642;
    let t16384 = t11374 * t16383;
    let t16387 = t3786 * t1409;
    let t16388 = t1961 * t1319;
    let t16389 = t16388 * t1419;
    let t16390 = t16387 * t16389;
    let t16393 = t5439 * t16073;
    (t16379, t16384, t16390, t16393)
}
