//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 848/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk848<F: Float>(t37406: F, t82: F, t37357: F, t7761: F, t89: F, t2999: F, t433: F, t1755: F, t1587: F, t27: F, t37360: F, t37365: F, t37368: F, t37372: F, t37376: F, t37379: F, t37383: F, t37386: F, t37394: F, t37399: F, t37403: F) -> (F, F, F, F, F) {
    let t37407 = t82 * t37406;
    let t37410 = t89 * t7761 * t37407 * t37357;
    let t37413 = t89 * t2999 * t433;
    let t37414 = F::cast_from(56.0_f64) / F::cast_from(81.0_f64) * t37413;
    let t37415 = t1755 * t1755;
    let t37418 = t89 * t27 * t1587 * t37415;
    let t37419 = -F::cast_from(40.0_f64) / F::cast_from(243.0_f64) * t37360 - t37365 / F::cast_from(9.0_f64) + F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t37368 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t37372 + t37376 / F::cast_from(3.0_f64) - F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t37379 + t37383 + t37386 - t37394 / F::cast_from(18.0_f64) - F::cast_from(6.0_f64) * t37399 + F::cast_from(20.0_f64) / F::cast_from(243.0_f64) * t37403 + F::cast_from(20.0_f64) / F::cast_from(27.0_f64) * t37410 + t37414 + t37418;
    (t37410, t37413, t37415, t37418, t37419)
}
