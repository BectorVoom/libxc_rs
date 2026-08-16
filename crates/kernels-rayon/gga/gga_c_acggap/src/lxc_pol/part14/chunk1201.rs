//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 1201/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk1201(t7426: f64, t8480: f64, t8605: f64, t31840: f64, t31849: f64, t31855: f64, t31868: f64, t36352: f64, t36354: f64, t36356: f64, t36365: f64, t36368: f64, t36373: f64, t36378: f64, t37970: f64, t37982: f64, t40533: f64, t40537: f64, t40542: f64, t40546: f64) -> f64 {
    let t40549 = t7426 * t8480 * t8605;
    let t40551 = 0.7862023072401038017e-3_f64 * t40533 + 0.7862023072401038017e-3_f64 * t40537 - t31840 + 0.7862023072401038017e-3_f64 * t31849 - 0.31448092289604152068e-2_f64 * t40542 - t37970 - t36352 + t36354 + t36356 + 0.17149607247227894789e-2_f64 * t31855 + t36365 + t36368 + t40546 / 96.0_f64 - t37982 - t36373 + t31868 - t36378 + 0.42874018118069736972e-3_f64 * t40549;
    t40551
}
