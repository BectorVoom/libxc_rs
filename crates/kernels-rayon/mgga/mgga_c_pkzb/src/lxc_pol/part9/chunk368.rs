//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 368/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk368(t1440: f64, t27: f64, t1419: f64, t1426: f64, t1432: f64, t1437: f64, t16: f64, t23: f64, t434: f64, t441: f64, t7: f64) -> (f64, f64) {
    let t1441 = t27 * t1440;
    let t1444 = 88.0_f64 / 9.0_f64 * t1419 * t16 - 80.0_f64 / 9.0_f64 * t434 * t441 + 10.0_f64 / 9.0_f64 * t7 * t1426 + 5.0_f64 / 3.0_f64 * t7 * t1432 + 10.0_f64 / 9.0_f64 * t23 * t1437 + 5.0_f64 / 3.0_f64 * t23 * t1441;
    (t1441, t1444)
}
