//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 978/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk978(t22373: f64, t6027: f64, t17382: f64, t21910: f64, t5908: f64, t22349: f64, t22352: f64, t22355: f64, t22359: f64, t22362: f64, t22365: f64, t22367: f64, t22369: f64, t22371: f64) -> (f64, f64, f64, f64) {
    let t22374 = t6027 * t22373;
    let t22376 = t17382 * t21910;
    let t22377 = t5908 * t22376;
    let t22379 = -t22349 / 128.0_f64 + t22352 / 4.0_f64 + t22355 / 288.0_f64 - t22359 / 16.0_f64 + t22362 / 8.0_f64 + t22365 / 192.0_f64 - t22367 / 18.0_f64 - t22369 / 8.0_f64 - t22371 / 18.0_f64 + t22374 / 12.0_f64 + t22377 / 54.0_f64;
    (t22374, t22376, t22377, t22379)
}
