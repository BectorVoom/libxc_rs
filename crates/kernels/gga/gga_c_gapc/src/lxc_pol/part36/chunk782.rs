//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 782/1133 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk782<F: Float>(t10381: F, t61: F, t3189: F, t132: F, t3186: F, t190: F, t329: F, t2536: F, t10343: F, t2405: F, t493: F, t3230: F, t6808: F, t996: F, t3231: F, t2300: F) -> (F, F, F, F, F, F) {
    let t10382 = t61 * t10381;
    let t10383 = t10382 * t3189;
    let t10385 = t132 * t3186;
    let t10386 = t10385 * t3189;
    let t10388 = t190 * t329;
    let t10389 = t10388 * t2536;
    let t10390 = t10343 * t10389;
    let t10392 = t493 * t2405;
    let t10393 = t3230 * t10392;
    let t10395 = t996 * t6808;
    let t10396 = t10395 * t3231;
    let t10398 = t493 * t2300;
    (t10383, t10386, t10390, t10393, t10396, t10398)
}
