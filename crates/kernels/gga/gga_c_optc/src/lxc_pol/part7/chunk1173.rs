//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1173/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1173<F: Float>(t24320: F, t24333: F, t265: F, t241: F, t2441: F, t7202: F, t2466: F, t7504: F, t2473: F, t7501: F, t845: F, t2248: F, t7207: F) -> (F, F, F, F, F) {
    let t24335 = (t24320 + t24333) * t265;
    let t24337 = F::new(0.19751789702565206229e-1) * t241 * t24335;
    let t24339 = F::new(0.14035736153892489771e2) * t2441 * t7202;
    let t24341 = t7504 * t2466;
    let t24344 = F::new(0.61523382126046769581e4) * t845 * t7501 * t2473 * t24341;
    let t24345 = t2248 * t7207;
    (t24335, t24337, t24339, t24344, t24345)
}
