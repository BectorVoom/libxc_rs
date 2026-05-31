//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 1101/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk1101<F: Float>(t5676: F, t570: F, t6171: F, t1750: F, t31824: F, t1988: F, t9573: F, t1089: F, t13067: F, t598: F, t9552: F, t30412: F, t30416: F, t30422: F, t30429: F, t30444: F, t30452: F, t30457: F, t30463: F, t34383: F, t34391: F, t37066: F, t37067: F) -> F {
    let t39254 = t570 * t5676;
    let t39256 = t570 * t6171;
    let t39262 = t31824 * t1750;
    let t39264 = t1988 * t9573;
    let t39268 = t598 * t1089 * t13067 * t9552;
    let t39270 = F::cast_from(0.31448092289604152067e-2_f64) * t30412 - F::cast_from(0.12579236915841660827e-2_f64) * t30416 + t30422 - t39254 / F::cast_from(96.0_f64) - t39256 / F::cast_from(48.0_f64) + t30429 - F::cast_from(0.7862023072401038017e-3_f64) * t30444 + F::cast_from(0.31448092289604152068e-3_f64) * t30452 - F::cast_from(0.45017719023973223821e-2_f64) * t30457 - F::cast_from(0.47172138434406228102e-3_f64) * t30463 + F::cast_from(0.34299214494455789578e-2_f64) * t39262 + t34383 - t34391 + t37066 - t37067 + F::cast_from(0.64311027177104605458e-3_f64) * t39264 + F::cast_from(0.64311027177104605458e-3_f64) * t39268;
    t39270
}
