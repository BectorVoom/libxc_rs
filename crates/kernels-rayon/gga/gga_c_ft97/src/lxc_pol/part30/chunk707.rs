//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 707/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk707(t29030: f64, t296: f64, t1901: f64, t193: f64, t24962: f64, t25194: f64, t25195: f64, t29247: f64, t29250: f64, t29253: f64, t29256: f64, t29261: f64, t29265: f64, t29270: f64, t29274: f64, t29278: f64, t446: f64, t89: f64) -> f64 {
    let t29281 = t296 * t29030;
    let t29284 = t24962 / 9.0_f64 + t446 * t29247 / 3.0_f64 + 2.0_f64 / 3.0_f64 * t446 * t29250 + 2.0_f64 / 3.0_f64 * t446 * t29253 + t1901 * t29256 / 9.0_f64 + t1901 * t29261 / 9.0_f64 - t25194 + t89 * t193 * t29265 / 3.0_f64 + t25195 / 9.0_f64 + 2.0_f64 / 3.0_f64 * t446 * t29270 - t446 * t29274 / 9.0_f64 + t446 * t29278 / 3.0_f64 - t446 * t29281 / 3.0_f64;
    t29284
}
