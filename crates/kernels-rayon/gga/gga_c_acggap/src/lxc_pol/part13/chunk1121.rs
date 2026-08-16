//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 1121/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk1121(t35390: f64, t1462: f64, t7614: f64, t1446: f64, t7605: f64, t1441: f64, t1456: f64, t31164: f64, t31166: f64, t31179: f64, t31186: f64, t31188: f64, t31193: f64, t31202: f64, t31210: f64, t35373: f64, t35380: f64, t35385: f64, t35388: f64) -> f64 {
    let t35391 = t35390 / 32.0_f64;
    let t35392 = t7614 * t1462;
    let t35393 = 0.24009450146119052704e-1_f64 * t35392;
    let t35394 = t7605 * t1446;
    let t35395 = 0.68598428988911579156e-2_f64 * t35394;
    let t35396 = t7605 * t1441;
    let t35397 = 0.68598428988911579156e-2_f64 * t35396;
    let t35398 = t7605 * t1456;
    let t35399 = 0.34299214494455789578e-2_f64 * t35398;
    let t35400 = t7605 * t1462;
    let t35402 = -0.10718504529517434243e-2_f64 * t31164 - 0.53592522647587171215e-3_f64 * t31166 - t35373 - 0.7145669686344956162e-3_f64 * t31179 + 0.64311027177104605458e-3_f64 * t31186 - 0.47172138434406228102e-2_f64 * t31188 + 0.42874018118069736972e-3_f64 * t31193 - t35380 - 0.62896184579208304136e-3_f64 * t31202 + 0.52413487149340253445e-3_f64 * t31210 + t35385 + t35388 + t35391 - t35393 - t35395 + t35397 - t35399 + 0.51448821741683684366e-2_f64 * t35400;
    t35402
}
