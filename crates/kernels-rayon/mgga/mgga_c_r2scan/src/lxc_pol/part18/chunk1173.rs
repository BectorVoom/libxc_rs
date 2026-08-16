//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1173/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1173(t39355: f64, t39358: f64, t39362: f64, t39396: f64, t39401: f64, t39404: f64, t39411: f64, t43009: f64, t43012: f64, t43015: f64, t43018: f64, t43021: f64) -> f64 {
    let t43023 = -0.14282990759302185292e-1_f64 * t39355 - 0.57131963037208741168e-1_f64 * t39358 - 0.10975748638225852664e0_f64 * t43009 - t39362 - 0.86682217400542685632e-1_f64 * t43012 + 0.2600466522016280569e0_f64 * t43015 + 0.86682217400542685632e-1_f64 * t43018 - 0.32927245914677557992e0_f64 * t43021 + t39396 - t39401 - t39404 + t39411;
    t43023
}
