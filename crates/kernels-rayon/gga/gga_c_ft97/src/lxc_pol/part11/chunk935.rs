//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 935/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk935(t2273: f64, t8640: f64, t70: f64, t8119: f64, t179: f64, t37355: f64, t2277: f64, t2271: f64, t37357: f64, t37362: f64, t37391: f64, t39402: f64, t39404: f64, t39413: f64, t39415: f64, t39417: f64, t39422: f64, t631: f64, t632: f64, t637: f64, t72: f64, t8624: f64, t8633: f64, t8660: f64, t8709: f64) -> f64 {
    let t39424 = t8640 * t2273;
    let t39430 = t70 * t8119;
    let t39431 = t179 * t37355;
    let t39436 = t8640 * t2277;
    let t39438 = -6.0_f64 * t631 * t637 * t8624 * t8709 + 12.0_f64 * t39402 - 8.0_f64 / 3.0_f64 * t39404 - 4.0_f64 * t631 * t72 * t8660 * t37357 - t631 * t72 * t2271 * t37362 + 8.0_f64 / 9.0_f64 * t39413 - 16.0_f64 / 81.0_f64 * t39415 - 8.0_f64 / 9.0_f64 * t631 * t8633 * t39417 * t37357 - 160.0_f64 / 81.0_f64 * t39422 - 20.0_f64 / 9.0_f64 * t39424 + t631 * t72 * t632 * t37391 / 6.0_f64 + 14.0_f64 / 81.0_f64 * t631 * t39430 * t39431 * t37357 + 10.0_f64 / 9.0_f64 * t39436;
    t39438
}
