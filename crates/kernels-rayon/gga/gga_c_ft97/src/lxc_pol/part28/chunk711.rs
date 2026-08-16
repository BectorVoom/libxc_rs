//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 711/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk711(t1882: f64, t6649: f64, t6641: f64, t379: f64, t569: f64, t6725: f64, t1901: f64, t23532: f64, t27239: f64, t27242: f64, t27246: f64, t27249: f64, t27253: f64, t27257: f64, t27260: f64, t27265: f64, t27269: f64, t27273: f64, t3281: f64, t446: f64) -> f64 {
    let t27276 = t1882 * t6649;
    let t27278 = t1882 * t6641;
    let t27281 = t569 * t6725 * t379;
    let t27285 = -2.0_f64 / 9.0_f64 * t1901 * t27239 - 2.0_f64 / 9.0_f64 * t1901 * t27242 + t1901 * t27246 / 9.0_f64 + t1901 * t27249 / 9.0_f64 - 2.0_f64 / 9.0_f64 * t1901 * t27253 - t1901 * t27257 / 9.0_f64 - t446 * t27260 / 3.0_f64 + t446 * t27265 / 3.0_f64 + 2.0_f64 / 3.0_f64 * t446 * t27269 - 2.0_f64 / 9.0_f64 * t3281 * t27273 + t27276 / 27.0_f64 - t27278 / 9.0_f64 - t446 * t27281 / 9.0_f64 - 2.0_f64 / 9.0_f64 * t23532;
    t27285
}
