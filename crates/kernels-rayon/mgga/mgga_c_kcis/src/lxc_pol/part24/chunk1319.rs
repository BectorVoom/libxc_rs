//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1319/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1319(t20853: f64, t2167: f64, t97601: f64, t101739: f64, t101740: f64, t1295: f64, t18444: f64, t20817: f64, t2169: f64, t233: f64, t235: f64, t236: f64, t29219: f64, t29226: f64, t441: f64, t6293: f64, t7673: f64, t915: f64, t92356: f64, t92360: f64, t92368: f64, t92375: f64) -> f64 {
    let t101750 = t20853 * t2167;
    let t101757 = 2.0_f64 * t97601;
    let t101761 = -t233 * t236 * (t101739 + t101740) / 16.0_f64 + t92356 - t2169 * t6293 * t1295 / 16.0_f64 - t92360 + t7673 * t29226 / 16.0_f64 + t92368 + t101750 - t92375 - t2169 * t18444 * t441 / 16.0_f64 - t233 * t915 * t29219 / 16.0_f64 + t101757 - t2169 * t235 * t20817 / 16.0_f64;
    t101761
}
