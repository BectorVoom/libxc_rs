//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 422/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk422(t1457: f64, t912: f64, t1289: f64, t929: f64, t926: f64, t1413: f64, t1427: f64, t1453: f64, t1455: f64) -> (f64, f64, f64, f64) {
    let t1459 = 0.5848223622634646207e0_f64 * t912 * t1457;
    let t1460 = t929 * t1289;
    let t1461 = t926 * t1460;
    let t1464 = -t1413 + t1427 + t1453 + t1455 - t1459;
    (t1459, t1460, t1461, t1464)
}
