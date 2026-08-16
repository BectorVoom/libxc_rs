//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 591/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk591(t1435: f64, t983: f64, t444: f64, t27: f64, t8: f64, t1429: f64, t23: f64, t2490: f64, t2494: f64, t434: f64, t445: f64, t7: f64, t974: f64, t980: f64) -> (f64, f64, f64, f64, f64) {
    let t2499 = t1435 * t983;
    let t2500 = t2499 * t444;
    let t2503 = t27 * t8;
    let t2504 = t2503 * t1429;
    let t2507 = -40.0_f64 / 9.0_f64 * t434 * t974 + 10.0_f64 / 9.0_f64 * t7 * t2490 + 5.0_f64 / 3.0_f64 * t7 * t2494 - 40.0_f64 / 9.0_f64 * t980 * t445 + 10.0_f64 / 9.0_f64 * t23 * t2500 - 5.0_f64 / 3.0_f64 * t23 * t2504;
    (t2499, t2500, t2503, t2504, t2507)
}
