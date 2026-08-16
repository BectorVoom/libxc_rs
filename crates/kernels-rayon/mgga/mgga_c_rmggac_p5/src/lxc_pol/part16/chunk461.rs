//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 461/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk461(t1412: f64, t2: f64, t428: f64, t1372: f64, t980: f64, t973: f64, t421: f64, t155: f64, t1439: f64, t453: f64, t1156: f64, t592: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t5462 = t1412 * t2;
    let t5464 = 0.36622894612013090108e-3_f64 * t5462 * t428;
    let t5465 = t1372 * t980;
    let t5467 = t1372 * t973;
    let t5469 = t1412 * t421;
    let t5471 = 2.0_f64 * t155 * t5469;
    let t5477 = t1439 * t453;
    let t5480 = t592 * t1156;
    (t5464, t5465, t5467, t5471, t5477, t5480)
}
