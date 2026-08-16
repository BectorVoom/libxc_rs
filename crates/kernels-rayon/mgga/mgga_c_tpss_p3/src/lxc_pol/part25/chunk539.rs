//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 539/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk539(t284: f64, t2453: f64, t872: f64, t876: f64, t301: f64, t875: f64, t296: f64, t2511: f64, t304: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t2532 = t284 * t284;
    let t2533 = 1.0_f64 / t2532;
    let t2537 = 0.22831111111111111111e-1_f64 * t2453;
    let t2545 = t872 * t876;
    let t2548 = t875 * t301;
    let t2549 = 1.0_f64 / t2548;
    let t2550 = t296 * t2549;
    let t2557 = 0.68863333333333333333e0_f64 * t2453;
    let t2564 = 0.17365833333333333333e0_f64 * t2511;
    let t2573 = t875 * t875;
    let t2574 = 1.0_f64 / t2573;
    let t2575 = t296 * t2574;
    let t2576 = t304 * t304;
    (t2532, t2533, t2537, t2545, t2549, t2550, t2557, t2564, t2573, t2574, t2575, t2576)
}
