//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 616/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk616(t3410: f64, t51: f64, t1721: f64, t592: f64, t1020: f64, t2600: f64, t179: f64, t2610: f64, t2608: f64, t2615: f64, t2617: f64, t2621: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3411 = t51 * t3410;
    let t3413 = t592 * t3411 * t1721;
    let t3417 = t2600 * t1020;
    let t3418 = t179 * t3417;
    let t3421 = 0.11696447245269292414e1_f64 * t2610;
    let t3422 = 2.0_f64 * t2608;
    let t3423 = 8.0_f64 * t2615;
    let t3424 = 8.0_f64 * t2617;
    let t3425 = 0.36622894612013090108e-3_f64 * t2621;
    (t3411, t3413, t3418, t3421, t3422, t3423, t3424, t3425)
}
