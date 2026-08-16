//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 116/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk116(t1: f64, t203: f64, t355: f64, t136: f64, t352: f64, t344: f64, t348: f64) -> (f64, f64, f64, f64) {
    let t357 = t355 * t1 * t203;
    let t358 = t352 * t136 * t357;
    let t359 = t358 / 24.0_f64;
    let t360 = -t344 - t348 / 4.0_f64 + t359;
    (t357, t358, t359, t360)
}
