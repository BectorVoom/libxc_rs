//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1196/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1196<F: Float>(t10918: F, t11497: F, t3262: F, t11506: F, t37342: F, t37431: F, t37438: F, t37443: F, t37444: F, t37448: F, t40327: F, t40329: F, t40331: F, t40334: F, t40338: F, t40342: F, t40346: F, t40348: F) -> (F, F, F) {
    let t40351 = F::new(3.0) / F::new(2.0) * t3262 * t10918 * t11497;
    let t40353 = F::new(3.0) / F::new(4.0) * t11506 * t37342;
    let t40355 = -F::cast_from(0.14408463291498358381e-2_f64) * t37431 + F::cast_from(0.20496175532535769484e-3_f64) * t37438 - t40327 - t40329 + F::cast_from(0.81300399444200075504e-3_f64) * t40331 - F::cast_from(0.1951603679568577289e-3_f64) * t40334 + t37443 + t40338 + t40342 - t40346 + t40348 + t40351 - t40353 + F::cast_from(0.60975299583150056628e-3_f64) * t37444 - t37448;
    (t40351, t40353, t40355)
}
