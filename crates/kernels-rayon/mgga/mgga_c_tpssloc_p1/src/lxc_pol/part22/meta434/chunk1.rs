//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1771/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1771(t19482: f64, t666: f64, t5468: f64, t9384: f64, t659: f64, t1444: f64, t2: f64, t584: f64, t2341: f64, t5396: f64, t9212: f64, t95: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t19483 = t19482 * t666;
    let t19488 = t9384 * t5468;
    let t19489 = t19488 * t659;
    let t19492 = t1444 * t2;
    let t19493 = t19492 * t584;
    let t19498 = t2341 * t5396;
    let t19499 = t19498 * t659;
    let t19503 = -t584 - 3.0_f64 * t9212;
    let t19504 = t95 * t19503;
    (t19483, t19489, t19493, t19499, t19503, t19504)
}
