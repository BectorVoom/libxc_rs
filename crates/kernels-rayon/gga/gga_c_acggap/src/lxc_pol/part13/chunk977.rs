//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 977/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk977(t104: f64, t9081: f64, t694: f64, t9090: f64, t467: f64, t9089: f64, t1268: f64, t560: f64, t9083: f64, t96: f64, t10409: f64, t10956: f64, t14974: f64, t19409: f64, t1954: f64, t2355: f64, t24623: f64, t2541: f64, t3984: f64, t567: f64, t7292: f64, t7297: f64, t8027: f64, t8382: f64, t9096: f64, t9097: f64) -> f64 {
    let t33352 = t104 * t9081;
    let t33357 = 6.0_f64 * t694 * t9090;
    let t33358 = t9089 * t467;
    let t33383 = t560 * t1268;
    let t33388 = 2.0_f64 * t96 * t9083;
    let t33389 = -6.0_f64 * t10409 * t7297 * t9089 - 6.0_f64 * t10956 * t3984 * t7297 - 3.0_f64 * t14974 * t2541 * t7297 - 6.0_f64 * t19409 * t2541 * t7297 + 6.0_f64 * t1954 * t33352 * t567 + 2.0_f64 * t2355 * t567 * t8027 + 6.0_f64 * t24623 * t7297 * t9097 + 12.0_f64 * t33358 * t7297 * t9097 + 2.0_f64 * t33383 * t9096 * t9097 + 6.0_f64 * t567 * t7292 * t8382 - t33357 + t33388;
    t33389
}
