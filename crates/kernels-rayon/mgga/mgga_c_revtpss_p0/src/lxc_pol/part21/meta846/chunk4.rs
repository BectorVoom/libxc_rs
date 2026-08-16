//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3169/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3169(t58225: f64, t56248: f64, t56252: f64, t56256: f64, t58202: f64, t58207: f64, t58209: f64, t58211: f64, t58214: f64, t58217: f64, t58220: f64, t58223: f64) -> f64 {
    let t58452 = 0.5519e0_f64 * t58225;
    let t58453 = 0.49671e0_f64 * t58202 + 0.10064166666666666667e1_f64 * t56248 + 0.543465e1_f64 * t56252 - 0.36231e1_f64 * t56256 - 0.73586666666666666668e-1_f64 * t58207 - 0.33114e0_f64 * t58209 - 0.99342e0_f64 * t58211 + 0.44152e0_f64 * t58214 + 0.16557e0_f64 * t58217 + 0.149013e1_f64 * t58220 + 0.198684e1_f64 * t58223 + t58452;
    t58453
}
