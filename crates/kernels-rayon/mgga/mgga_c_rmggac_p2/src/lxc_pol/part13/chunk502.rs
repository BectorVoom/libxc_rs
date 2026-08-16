//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 502/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk502(t4366: f64, t4368: f64, t4370: f64, t1412: f64, t2: f64, t428: f64, t1372: f64, t980: f64, t973: f64, t421: f64, t155: f64, t4324: f64, t4328: f64, t4361: f64, t4365: f64, t5435: f64, t5445: f64, t5447: f64, t5449: f64, t5451: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t5459 = 16.0_f64 * t4366;
    let t5460 = 4.0_f64 * t4368;
    let t5461 = 4.0_f64 * t4370;
    let t5462 = t1412 * t2;
    let t5464 = 0.36622894612013090108e-3_f64 * t5462 * t428;
    let t5465 = t1372 * t980;
    let t5466 = 0.11696447245269292414e1_f64 * t5465;
    let t5467 = t1372 * t973;
    let t5468 = 0.5848223622634646207e0_f64 * t5467;
    let t5469 = t1412 * t421;
    let t5471 = 2.0_f64 * t155 * t5469;
    let t5472 = t5435 + t4361 - t4365 + t5445 + t5447 + t5449 - t5451 + t4324 - t5459 - t5460 - t5461 + t4328 - t5464 + t5466 - t5468 + t5471;
    (t5459, t5460, t5461, t5464, t5466, t5468, t5471, t5472)
}
