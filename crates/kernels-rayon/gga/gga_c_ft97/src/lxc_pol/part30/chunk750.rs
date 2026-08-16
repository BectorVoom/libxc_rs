//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 750/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk750(t33476: f64, t505: f64, t2354: f64, t446: f64, t33253: f64, t713: f64, t193: f64, t89: f64, t33452: f64, t676: f64, t27: f64, t33340: f64, t33344: f64, t33349: f64, t33455: f64, t33459: f64, t33463: f64, t33467: f64, t33471: f64, t33475: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t33477 = t33476 * t505;
    let t33478 = t2354 * t33477;
    let t33479 = t446 * t33478;
    let t33481 = t33253 * t713;
    let t33482 = t193 * t33481;
    let t33483 = t89 * t33482;
    let t33485 = t676 * t33452;
    let t33487 = t89 * t27 * t33485;
    let t33488 = t33340 + t33344 / 6.0_f64 + t33349 - t33455 / 2.0_f64 - t33459 - 2.0_f64 / 3.0_f64 * t33463 - 6.0_f64 * t33467 + 4.0_f64 * t33471 + t33475 + t33479 / 3.0_f64 + 2.0_f64 * t33483 - t33487;
    (t33478, t33479, t33481, t33483, t33485, t33487, t33488)
}
