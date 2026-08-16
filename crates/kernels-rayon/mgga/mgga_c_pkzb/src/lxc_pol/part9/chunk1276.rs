//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1276/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1276(t2328: f64, t8013: f64, t2336: f64, t8028: f64, t2192: f64, t8004: f64, t8017: f64, t1185: f64, t6142: f64, t6143: f64, t1306: f64, t22359: f64, t22361: f64, t22363: f64, t22366: f64, t22374: f64, t2464: f64, t8563: f64, t955: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t22376 = 0.10389515463408878255e3_f64 * t2328 * t8013;
    let t22378 = 0.17544670867903938621e1_f64 * t8028 * t2336;
    let t22380 = 3.0_f64 * t2192 * t8004;
    let t22382 = 0.31168546390226634765e3_f64 * t2328 * t8017;
    let t22385 = 24.0_f64 * t6142 * t1185 * t6143;
    let t22386 = -3.0_f64 * t1306 * t2464 * t8563 * t955 + t22359 + t22361 - t22363 - t22366 - t22374 - t22376 - t22378 + t22380 + t22382 - t22385;
    (t22376, t22378, t22380, t22382, t22385, t22386)
}
