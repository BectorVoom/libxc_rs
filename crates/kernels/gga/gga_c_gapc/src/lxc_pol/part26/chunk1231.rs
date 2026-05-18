//! GGA_C_GAPC lxc pol — lxc_pol part 26 (v4rho2sigma2_5) CSE chunk 1231/1308 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part26_v4rho2sigma2_5_chunk1231<F: Float>(t11387: F, t21053: F, t21054: F, t11365: F, t5285: F, t5703: F, t35203: F, t35205: F, t35210: F, t35212: F, t35215: F, t35217: F, t35222: F, t35225: F, t35228: F) -> F {
    let t35231 = t21053 * t11387 * t21054;
    let t35234 = t5285 * t11365 * t5703;
    let t35236 = F::new(0.36231816839129402172e-6) * t35203 + F::new(0.1374296967252737644e-5) * t35205 + F::new(0.92386400563397210585e-6) * t35210 - F::new(0.40096157891080460192e-6) * t35212 - F::new(0.40022999988963401106e-7) * t35215 + F::new(0.51491428373437201896e-5) * t35217 - F::new(0.75091666377929252765e-6) * t35222 + F::new(0.30353495895471971564e-6) * t35225 + F::new(0.21720231316129303386e-4) * t35228 + F::new(0.5686343261418565457e-6) * t35231 - F::new(0.52838066223730378166e-7) * t35234;
    t35236
}
