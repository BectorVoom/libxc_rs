//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 620/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk620(t8381: f64, t8473: f64, t8525: f64, t8586: f64, t103: f64, t8460: f64, t108: f64, t1538: f64, t1761: f64, t1920: f64, t438: f64, t497: f64, t7734: f64, t7736: f64, t8199: f64, t8356: f64, t8361: f64, t8364: f64, t8420: f64, t8467: f64, t8502: f64, t88: f64) -> (f64, f64, f64) {
    let t8588 = t8381 + t8473 + t8525 + t8586;
    let t8590 = t8460 * t103;
    let t8598 = -t108 * t7734 - 2.0_f64 * t108 * t7736 - t108 * t8199 - 3.0_f64 * t1538 * t497 - 3.0_f64 * t1761 * t497 - 3.0_f64 * t1920 * t438 - t8588 * t88 - 2.0_f64 * t8356 - 6.0_f64 * t8361 - 6.0_f64 * t8364 - 12.0_f64 * t8420 + 12.0_f64 * t8467 + 12.0_f64 * t8502 + 2.0_f64 * t8590;
    (t8588, t8590, t8598)
}
