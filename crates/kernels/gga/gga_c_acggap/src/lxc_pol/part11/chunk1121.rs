//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 1121/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk1121<F: Float>(t35390: F, t1462: F, t7614: F, t1446: F, t7605: F, t1441: F, t1456: F, t31164: F, t31166: F, t31179: F, t31186: F, t31188: F, t31193: F, t31202: F, t31210: F, t35373: F, t35380: F, t35385: F, t35388: F) -> F {
    let t35391 = t35390 / F::cast_from(32.0_f64);
    let t35392 = t7614 * t1462;
    let t35393 = F::cast_from(0.24009450146119052704e-1_f64) * t35392;
    let t35394 = t7605 * t1446;
    let t35395 = F::cast_from(0.68598428988911579156e-2_f64) * t35394;
    let t35396 = t7605 * t1441;
    let t35397 = F::cast_from(0.68598428988911579156e-2_f64) * t35396;
    let t35398 = t7605 * t1456;
    let t35399 = F::cast_from(0.34299214494455789578e-2_f64) * t35398;
    let t35400 = t7605 * t1462;
    let t35402 = -F::cast_from(0.10718504529517434243e-2_f64) * t31164 - F::cast_from(0.53592522647587171215e-3_f64) * t31166 - t35373 - F::cast_from(0.7145669686344956162e-3_f64) * t31179 + F::cast_from(0.64311027177104605458e-3_f64) * t31186 - F::cast_from(0.47172138434406228102e-2_f64) * t31188 + F::cast_from(0.42874018118069736972e-3_f64) * t31193 - t35380 - F::cast_from(0.62896184579208304136e-3_f64) * t31202 + F::cast_from(0.52413487149340253445e-3_f64) * t31210 + t35385 + t35388 + t35391 - t35393 - t35395 + t35397 - t35399 + F::cast_from(0.51448821741683684366e-2_f64) * t35400;
    t35402
}
