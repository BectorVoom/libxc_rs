//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 647/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk647(t1901: f64, t23176: f64, t26276: f64, t26280: f64, t26284: f64, t26288: f64, t26291: f64, t26293: f64, t26295: f64, t26297: f64, t26301: f64, t26303: f64, t26306: f64, t26309: f64, t26312: f64, t446: f64) -> f64 {
    let t26315 = -t446 * t26276 / 3.0_f64 - t446 * t26280 / 3.0_f64 - t446 * t26284 / 3.0_f64 - t446 * t26288 / 3.0_f64 + t26291 / 9.0_f64 + t26293 / 9.0_f64 + t26295 / 9.0_f64 + 2.0_f64 / 27.0_f64 * t1901 * t26297 - t23176 / 9.0_f64 - t26301 / 9.0_f64 - 2.0_f64 / 9.0_f64 * t26303 - t1901 * t26306 / 9.0_f64 - 2.0_f64 / 9.0_f64 * t1901 * t26309 - 2.0_f64 / 9.0_f64 * t1901 * t26312;
    t26315
}
