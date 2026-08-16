//! GGA_C_GAPC lxc pol — lxc_pol part 27 (v4rho2sigma2_6) CSE chunk 1234/1310 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part27_v4rho2sigma2_6_chunk1234<F: Float>(t11387: F, t21053: F, t21054: F, t11365: F, t5285: F, t5703: F, t35203: F, t35205: F, t35210: F, t35212: F, t35215: F, t35217: F, t35222: F, t35225: F, t35228: F) -> F {
    let t35231 = t21053 * t11387 * t21054;
    let t35234 = t5285 * t11365 * t5703;
    let t35236 = F::cast_from(0.36231816839129402172e-6_f64) * t35203 + F::cast_from(0.1374296967252737644e-5_f64) * t35205 + F::cast_from(0.92386400563397210585e-6_f64) * t35210 - F::cast_from(0.40096157891080460192e-6_f64) * t35212 - F::cast_from(0.40022999988963401106e-7_f64) * t35215 + F::cast_from(0.51491428373437201896e-5_f64) * t35217 - F::cast_from(0.75091666377929252765e-6_f64) * t35222 + F::cast_from(0.30353495895471971564e-6_f64) * t35225 + F::cast_from(0.21720231316129303386e-4_f64) * t35228 + F::cast_from(0.5686343261418565457e-6_f64) * t35231 - F::cast_from(0.52838066223730378166e-7_f64) * t35234;
    t35236
}
