//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 388/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk388(t1279: f64, t1281: f64, t547: f64, t548: f64, t553: f64, t557: f64, t561: f64, t565: f64, t569: f64, t573: f64, t4: f64, t579: f64) -> (f64, f64, f64) {
    let t1284 = t1279 * t548 + 3.0_f64 * t1281 * t547;
    let t1286 = -t553 - t557 - t561 - t565 - t569 - t573;
    let t1288 = -t4 - t579;
    (t1284, t1286, t1288)
}
