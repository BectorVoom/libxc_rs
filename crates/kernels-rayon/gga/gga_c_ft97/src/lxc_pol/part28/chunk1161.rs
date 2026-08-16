//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1161/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1161(t139312: f64, t139991: f64, t139992: f64, t148381: f64, t148385: f64, t148388: f64, t148392: f64, t148396: f64, t148401: f64, t148405: f64, t148410: f64, t148414: f64, t148419: f64, t148422: f64, t148426: f64, t148430: f64) -> f64 {
    let t148781 = -3.0_f64 / 8.0_f64 * t148381 - 3.0_f64 * t148385 - t148388 / 2.0_f64 + 2.0_f64 * t148392 + 4.0_f64 * t148396 - t139312 / 3.0_f64 + 3.0_f64 / 4.0_f64 * t148401 + t139991 - t139992 - 8.0_f64 / 3.0_f64 * t148405 + 4.0_f64 / 3.0_f64 * t148410 - 4.0_f64 / 3.0_f64 * t148414 + 2.0_f64 / 9.0_f64 * t148419 + 4.0_f64 / 3.0_f64 * t148422 - 6.0_f64 * t148426 + t148430 / 6.0_f64;
    t148781
}
