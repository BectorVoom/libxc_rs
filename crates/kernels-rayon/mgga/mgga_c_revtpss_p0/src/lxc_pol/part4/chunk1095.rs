//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1095/1428 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk1095(t4264: f64, t625: f64, t4288: f64, t10208: f64, t1513: f64, t2340: f64, t2339: f64, t4287: f64, t665: f64, t2366: f64, t4263: f64, t10227: f64, t1504: f64, t2350: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t13451 = 4.0_f64 / 3.0_f64 * t625 * t4264;
    let t13453 = 2.0_f64 / 3.0_f64 * t625 * t4288;
    let t13455 = t10208 * t1513 * t2340;
    let t13458 = t2339 * t4287;
    let t13459 = t13458 * t665;
    let t13462 = t4263 * t2366;
    let t13472 = t10227 * t1504 * t2350;
    (t13451, t13453, t13455, t13459, t13462, t13472)
}
