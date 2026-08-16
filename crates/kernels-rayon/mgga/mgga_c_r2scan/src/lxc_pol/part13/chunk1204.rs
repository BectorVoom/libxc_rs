//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1204/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1204(t11020: f64, t11483: f64, t11626: f64, t37271: f64, t11476: f64, t37282: f64, t11519: f64, t11563: f64, t2312: f64, t3446: f64, t3447: f64, t158: f64, t2461: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t40443 = t11020 * t11483 / 4.0_f64;
    let t40444 = t37271 * t11626;
    let t40446 = 3.0_f64 / 2.0_f64 * t37282 * t11476;
    let t40448 = 15.0_f64 / 8.0_f64 * t37282 * t11519;
    let t40451 = t3446 * t3447 * t11563 * t2312;
    let t40453 = t158 * t2461;
    (t40443, t40444, t40446, t40448, t40451, t40453)
}
