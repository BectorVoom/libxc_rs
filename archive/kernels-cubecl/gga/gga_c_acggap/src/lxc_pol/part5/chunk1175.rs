//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1175/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1175<F: Float>(t1140: F, t6320: F, t1137: F, t5907: F, t145: F, t5506: F, t1150: F, t1318: F, t16425: F, t16427: F, t16438: F, t16440: F, t16442: F, t16444: F, t16446: F, t18129: F, t301: F, t335: F, t6387: F, t839: F, t960: F) -> F {
    let t21303 = t1140 * t6320;
    let t21305 = t1137 * t5907;
    let t21307 = t145 * t5506;
    let t21319 = F::cast_from(0.80031500487063509016e-2_f64) * t16425 - F::cast_from(0.34299214494455789578e-2_f64) * t16427 + F::cast_from(0.25724410870841842183e-2_f64) * t16438 + F::cast_from(0.68598428988911579156e-2_f64) * t16440 - F::cast_from(0.68598428988911579156e-2_f64) * t16442 + F::cast_from(0.68598428988911579156e-2_f64) * t16444 + F::cast_from(0.68598428988911579156e-2_f64) * t16446 - F::cast_from(7.0_f64) / F::cast_from(36.0_f64) * t21303 - F::cast_from(7.0_f64) / F::cast_from(72.0_f64) * t21305 + t1150 * t960 * t21307 * t301 / F::cast_from(8.0_f64) + t1150 * t960 * t6387 * t839 / F::cast_from(16.0_f64) + t335 * t18129 * t1318 / F::cast_from(12.0_f64);
    t21319
}
