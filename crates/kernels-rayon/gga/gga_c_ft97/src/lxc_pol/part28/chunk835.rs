//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 835/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk835(t34379: f64, t8270: f64, t1317: f64, t28: f64, t1800: f64, t34384: f64, t7211: f64, t965: f64, t469: f64, t5665: f64, t32435: f64, t32440: f64, t34373: f64, t34377: f64, t34382: f64, t34387: f64, t34391: f64, t34395: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t34397 = t8270 * t34379;
    let t34399 = t1317 * t28 * t34397;
    let t34401 = t1800 * t34384;
    let t34403 = t1317 * t28 * t34401;
    let t34405 = t7211 * t965;
    let t34406 = t469 * t34405;
    let t34408 = t5665 * t28 * t34406;
    let t34410 = 3.0_f64 / 2.0_f64 * t34373 + t32435 + 2.0_f64 / 3.0_f64 * t34377 + 4.0_f64 * t34382 - 2.0_f64 * t34387 - t34391 / 2.0_f64 - t32440 - t34395 / 3.0_f64 - 3.0_f64 * t34399 + 2.0_f64 * t34403 + t34408 / 4.0_f64;
    (t34397, t34399, t34401, t34403, t34405, t34406, t34408, t34410)
}
