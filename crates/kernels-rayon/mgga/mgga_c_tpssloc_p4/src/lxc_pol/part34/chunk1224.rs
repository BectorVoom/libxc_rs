//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1224/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1224(t105278: f64, t105282: f64, t105286: f64, t105288: f64, t105290: f64, t105292: f64, t105294: f64, t105296: f64, t105299: f64, t105304: f64, t84857: f64, t84859: f64, t87213: f64, t87243: f64, t98618: f64, t98647: f64, t98690: f64, t98694: f64, t98696: f64) -> f64 {
    let t108249 = -7.0_f64 / 48.0_f64 * t98618 + 0.12111826828242117256e-2_f64 * t98647 - t84857 + 0.72670960969452703536e-2_f64 * t105278 + 0.72670960969452703536e-2_f64 * t105282 - 0.14534192193890540707e-1_f64 * t105286 + t105288 / 64.0_f64 + t105290 / 32.0_f64 + t105292 / 64.0_f64 + t105294 / 128.0_f64 - t105296 / 256.0_f64 + t84859 + t105299 / 768.0_f64 + 0.10093189023535097713e-3_f64 * t87213 - 7.0_f64 / 384.0_f64 * t98690 - 119.0_f64 / 1152.0_f64 * t87243 - t105304 / 32.0_f64 + 7.0_f64 / 24.0_f64 * t98694 + 0.50869672678616892474e-1_f64 * t98696;
    t108249
}
