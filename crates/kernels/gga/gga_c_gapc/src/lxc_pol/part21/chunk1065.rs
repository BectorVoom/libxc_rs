//! GGA_C_GAPC lxc pol — lxc_pol part 21 (v4rho2sigma2_0) CSE chunk 1065/1125 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part21_v4rho2sigma2_0_chunk1065<F: Float>(t35203: F, t35205: F, t35210: F, t35212: F, t35215: F, t35217: F, t35222: F, t35225: F, t35228: F, t35231: F, t35234: F, t1386: F, t3663: F, t3665: F, t2981: F, t34754: F, t458: F) -> (F, F, F) {
    let t35236 = 0.36231816839129402172e-6 * t35203 + 0.1374296967252737644e-5 * t35205 + 0.92386400563397210585e-6 * t35210 - 0.40096157891080460192e-6 * t35212 - 0.40022999988963401106e-7 * t35215 + 0.51491428373437201896e-5 * t35217 - 0.75091666377929252765e-6 * t35222 + 0.30353495895471971564e-6 * t35225 + 0.21720231316129303386e-4 * t35228 + 0.5686343261418565457e-6 * t35231 - 0.52838066223730378166e-7 * t35234;
    let t35240 = t1386 * t3663 * t3665;
    let t35243 = t34754 * t2981 * t458;
    (t35236, t35240, t35243)
}
