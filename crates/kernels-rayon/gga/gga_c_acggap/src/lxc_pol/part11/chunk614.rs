//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 614/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk614(t1165: f64, t1586: f64, t407: f64, t1562: f64, t3379: f64, t1567: f64, t1466: f64, t3382: f64, t157: f64, t839: f64, t1532: f64, t1077: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t4695 = t1165 * t1586 * t407;
    let t4699 = 0.17149607247227894789e-2_f64 * t3379 * t1562;
    let t4701 = t1165 * t1567 * t407;
    let t4705 = 0.85748036236139473944e-3_f64 * t3382 * t1466;
    let t4706 = t157 * t839;
    let t4708 = t1165 * t1532 * t4706;
    let t4711 = t157 * t1077;
    (t4695, t4699, t4701, t4705, t4708, t4711)
}
