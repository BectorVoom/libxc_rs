//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 611/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk611(t8445: f64, t8459: f64, t103: f64, t82: f64, t1851: f64, t480: f64, t1853: f64, t83: f64, t1827: f64, t1882: f64, t1901: f64, t28: f64, t446: f64, t8383: f64, t8388: f64, t8393: f64, t8396: f64, t8399: f64, t8402: f64, t8406: f64, t8409: f64, t8413: f64, t8421: f64, t8426: f64, t8430: f64, t89: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t8460 = t8445 + t8459;
    let t8462 = t82 * t8460 * t103;
    let t8466 = t480 * t1851;
    let t8467 = t8466 * t1853;
    let t8468 = t83 * t8467;
    let t8471 = t1882 * t1827;
    let t8473 = 2.0_f64 / 3.0_f64 * t446 * t8383 + t1901 * t8388 / 3.0_f64 - 2.0_f64 / 9.0_f64 * t8393 - t446 * t8396 - t446 * t8399 - t446 * t8402 / 3.0_f64 - 2.0_f64 / 3.0_f64 * t446 * t8406 - 2.0_f64 / 3.0_f64 * t8409 - 2.0_f64 * t446 * t8413 - 2.0_f64 * t446 * t8421 - 2.0_f64 / 3.0_f64 * t1901 * t8426 - t8430 / 3.0_f64 + t89 * t28 * t8462 / 3.0_f64 + 2.0_f64 * t446 * t8468 + 2.0_f64 / 3.0_f64 * t8471;
    (t8460, t8462, t8466, t8467, t8468, t8473)
}
