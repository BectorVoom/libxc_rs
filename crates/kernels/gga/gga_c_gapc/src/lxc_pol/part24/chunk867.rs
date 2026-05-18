//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 867/1327 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk867<F: Float>(t10385: F, t3189: F, t190: F, t329: F, t2536: F, t10343: F, t2405: F, t493: F, t3230: F, t6808: F, t996: F, t3231: F) -> (F, F, F, F) {
    let t10386 = t10385 * t3189;
    let t10388 = t190 * t329;
    let t10389 = t10388 * t2536;
    let t10390 = t10343 * t10389;
    let t10392 = t493 * t2405;
    let t10393 = t3230 * t10392;
    let t10395 = t996 * t6808;
    let t10396 = t10395 * t3231;
    (t10386, t10390, t10393, t10396)
}
