//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 848/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk848(t37406: f64, t82: f64, t37357: f64, t7761: f64, t89: f64, t2999: f64, t433: f64, t1755: f64, t1587: f64, t27: f64, t37360: f64, t37365: f64, t37368: f64, t37372: f64, t37376: f64, t37379: f64, t37383: f64, t37386: f64, t37394: f64, t37399: f64, t37403: f64) -> (f64, f64, f64, f64, f64) {
    let t37407 = t82 * t37406;
    let t37410 = t89 * t7761 * t37407 * t37357;
    let t37413 = t89 * t2999 * t433;
    let t37414 = 56.0_f64 / 81.0_f64 * t37413;
    let t37415 = t1755 * t1755;
    let t37418 = t89 * t27 * t1587 * t37415;
    let t37419 = -40.0_f64 / 243.0_f64 * t37360 - t37365 / 9.0_f64 + 4.0_f64 / 9.0_f64 * t37368 + 4.0_f64 / 3.0_f64 * t37372 + t37376 / 3.0_f64 - 8.0_f64 / 27.0_f64 * t37379 + t37383 + t37386 - t37394 / 18.0_f64 - 6.0_f64 * t37399 + 20.0_f64 / 243.0_f64 * t37403 + 20.0_f64 / 27.0_f64 * t37410 + t37414 + t37418;
    (t37410, t37413, t37415, t37418, t37419)
}
