//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 1961/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1961(t3215: f64, t1406: f64, t9238: f64, t2239: f64, t3951: f64, t193: f64, t776: f64, t111: f64, t5363: f64, t6470: f64, t19297: f64, t604: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t43636 = t3215 * t3215;
    let t43637 = 1.0_f64 / t43636;
    let t45844 = t1406 * t9238;
    let t46104 = t3951 * t2239;
    let t46341 = t193 * t776;
    let t55353 = t5363 * t111;
    let t55388 = t6470 * t111;
    let t55880 = t19297 * t604;
    (t43637, t45844, t46104, t46341, t55353, t55388, t55880)
}
