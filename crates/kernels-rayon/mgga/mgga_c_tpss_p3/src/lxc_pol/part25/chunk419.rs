//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 419/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk419(t1441: f64, t318: f64, t1409: f64, t1416: f64, t1419: f64, t1422: f64, t898: f64, t901: f64) -> (f64, f64) {
    let t1442 = t1441 * t318;
    let t1448 = 0.258925e1_f64 * t1416 - t898 - 0.301925e0_f64 * t1409 + 0.16504875e0_f64 * t1419 - t901 - 0.82785e-1_f64 * t1422;
    (t1442, t1448)
}
