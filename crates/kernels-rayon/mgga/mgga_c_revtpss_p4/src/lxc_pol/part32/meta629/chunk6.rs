//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 2025/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2025(t30341: f64, t686: f64, t72: f64, t25375: f64, t28314: f64, t99463: f64, t27213: f64, t28360: f64, t103370: f64, t103382: f64, t103391: f64, t103393: f64, t103394: f64, t103396: f64, t103399: f64, t106404: f64, t18663: f64, t2067: f64, t25391: f64, t29682: f64, t7403: f64, t95825: f64, t95859: f64, t95862: f64) -> (f64, f64) {
    let t110502 = t30341 * t72 * t686;
    let t110503 = t25375 * t110502;
    let t110505 = t99463 * t28314;
    let t110517 = t27213 * t28360;
    let t110519 = -0.13009920719177044025e-2_f64 * t103370 - 0.28912093960683998207e-1_f64 * t110503 + 0.51405703062096148813e-1_f64 * t110505 - 0.39512695097613069591e1_f64 * t7403 * t18663 + t103382 + 0.17135234354032049604e-1_f64 * t95859 - 0.4336814094102599731e0_f64 * t106404 * t2067 - t95862 - t103391 + t103393 - 0.17347256376410398924e1_f64 * t25391 * t95825 * t29682 - 0.45699670022203476294e-2_f64 * t103394 + 0.39029762157531132076e-1_f64 * t103396 - t103399 + 0.14456046980341999104e-1_f64 * t110517;
    (t110502, t110519)
}
