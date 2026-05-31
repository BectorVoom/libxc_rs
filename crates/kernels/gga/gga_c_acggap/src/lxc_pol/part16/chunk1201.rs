//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 1201/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk1201<F: Float>(t7426: F, t8480: F, t8605: F, t31840: F, t31849: F, t31855: F, t31868: F, t36352: F, t36354: F, t36356: F, t36365: F, t36368: F, t36373: F, t36378: F, t37970: F, t37982: F, t40533: F, t40537: F, t40542: F, t40546: F) -> F {
    let t40549 = t7426 * t8480 * t8605;
    let t40551 = F::cast_from(0.7862023072401038017e-3_f64) * t40533 + F::cast_from(0.7862023072401038017e-3_f64) * t40537 - t31840 + F::cast_from(0.7862023072401038017e-3_f64) * t31849 - F::cast_from(0.31448092289604152068e-2_f64) * t40542 - t37970 - t36352 + t36354 + t36356 + F::cast_from(0.17149607247227894789e-2_f64) * t31855 + t36365 + t36368 + t40546 / F::cast_from(96.0_f64) - t37982 - t36373 + t31868 - t36378 + F::cast_from(0.42874018118069736972e-3_f64) * t40549;
    t40551
}
