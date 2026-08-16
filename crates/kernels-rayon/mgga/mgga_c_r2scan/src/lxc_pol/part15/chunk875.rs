//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 875/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk875(t1554: f64, t2124: f64, t2550: f64, t2294: f64, t2583: f64, t2582: f64, t1551: f64, t2572: f64, t360: f64, t113: f64, t1234: f64, t6063: f64, t7605: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t7934 = t2124 * t2550 * t1554;
    let t7937 = t2294 * t2583;
    let t7939 = 0.23115257973478049502e0_f64 * t2582 * t7937;
    let t7940 = t2572 * t1551;
    let t7941 = t360 * t7940;
    let t7944 = t113 * t1234;
    let t7945 = t2572 * t7944;
    let t7946 = t360 * t7945;
    let t7949 = t6063 * t7605;
    (t7934, t7939, t7940, t7941, t7944, t7945, t7946, t7949)
}
