//! GGA_C_GAPC lxc pol — lxc_pol part 38 (v4rho2sigma2_17) CSE chunk 1230/1307 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part38_v4rho2sigma2_17_chunk1230(t11387: f64, t21053: f64, t21054: f64, t11365: f64, t5285: f64, t5703: f64, t35203: f64, t35205: f64, t35210: f64, t35212: f64, t35215: f64, t35217: f64, t35222: f64, t35225: f64, t35228: f64) -> f64 {
    let t35231 = t21053 * t11387 * t21054;
    let t35234 = t5285 * t11365 * t5703;
    let t35236 = 0.36231816839129402172e-6_f64 * t35203 + 0.1374296967252737644e-5_f64 * t35205 + 0.92386400563397210585e-6_f64 * t35210 - 0.40096157891080460192e-6_f64 * t35212 - 0.40022999988963401106e-7_f64 * t35215 + 0.51491428373437201896e-5_f64 * t35217 - 0.75091666377929252765e-6_f64 * t35222 + 0.30353495895471971564e-6_f64 * t35225 + 0.21720231316129303386e-4_f64 * t35228 + 0.5686343261418565457e-6_f64 * t35231 - 0.52838066223730378166e-7_f64 * t35234;
    t35236
}
