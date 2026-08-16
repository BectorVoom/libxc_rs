//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 605/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk605(t3194: f64, t8376: f64, t3193: f64, t1901: f64, t446: f64, t8207: f64, t8213: f64, t8220: f64, t8224: f64, t8227: f64, t8229: f64, t8233: f64, t8235: f64, t8238: f64, t8357: f64, t8362: f64, t8365: f64, t8369: f64, t8373: f64) -> (f64, f64, f64) {
    let t8377 = t3194 * t8376;
    let t8378 = t3193 * t8377;
    let t8381 = -2.0_f64 / 3.0_f64 * t1901 * t8207 + 2.0_f64 / 9.0_f64 * t1901 * t8213 - 2.0_f64 / 3.0_f64 * t1901 * t8220 - t446 * t8224 / 3.0_f64 - 2.0_f64 / 9.0_f64 * t8227 - 2.0_f64 / 3.0_f64 * t8229 - 4.0_f64 / 27.0_f64 * t8233 + 2.0_f64 / 27.0_f64 * t8235 + 4.0_f64 / 9.0_f64 * t446 * t8238 - t446 * t8357 / 3.0_f64 - t446 * t8362 - t446 * t8365 - 2.0_f64 / 3.0_f64 * t1901 * t8369 + 2.0_f64 / 3.0_f64 * t1901 * t8373 - 2.0_f64 / 9.0_f64 * t1901 * t8378;
    (t8377, t8378, t8381)
}
