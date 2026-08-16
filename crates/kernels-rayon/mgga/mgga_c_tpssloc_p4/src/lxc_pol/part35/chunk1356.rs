//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1356/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1356(t20882: f64, t23146: f64, t20988: f64, t25084: f64, t20891: f64, t1898: f64, t20937: f64, t249: f64, t20983: f64, t105278: f64, t105282: f64, t105286: f64, t105288: f64, t105290: f64, t81736: f64, t81743: f64, t87213: f64, t87243: f64, t98618: f64, t98647: f64, t98690: f64, t98694: f64, t98696: f64) -> f64 {
    let t105292 = t23146 * t20882;
    let t105294 = t25084 * t20988;
    let t105296 = t23146 * t20891;
    let t105299 = t20937 * t1898 * t249;
    let t105304 = t25084 * t20983;
    let t105308 = -7.0_f64 / 96.0_f64 * t98618 + 0.60559134141210586281e-3_f64 * t98647 - t81736 + 0.36335480484726351768e-2_f64 * t105278 + 0.36335480484726351768e-2_f64 * t105282 - 0.72670960969452703536e-2_f64 * t105286 + t105288 / 128.0_f64 + t105290 / 64.0_f64 + t105292 / 128.0_f64 + t105294 / 256.0_f64 - t105296 / 512.0_f64 + t81743 + t105299 / 1536.0_f64 + 0.50465945117675488567e-4_f64 * t87213 - 7.0_f64 / 768.0_f64 * t98690 - 119.0_f64 / 2304.0_f64 * t87243 - t105304 / 64.0_f64 + 7.0_f64 / 48.0_f64 * t98694 + 0.25434836339308446238e-1_f64 * t98696;
    t105308
}
