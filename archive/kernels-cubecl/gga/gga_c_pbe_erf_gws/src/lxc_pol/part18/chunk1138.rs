//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1138/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1138<F: Float>(t2503: F, t3952: F, t14031: F, t3224: F, t3113: F, t4023: F, t3283: F, t4043: F, t14011: F, t3242: F, t3237: F, t3120: F) -> (F, F, F, F, F, F, F) {
    let t14479 = t3952 * t2503;
    let t14481 = t14031 * t3224;
    let t14483 = t3113 * t4023;
    let t14485 = t4043 * t3283;
    let t14487 = t14011 * t3242;
    let t14489 = t14011 * t3237;
    let t14491 = t3120 * t4023;
    (t14479, t14481, t14483, t14485, t14487, t14489, t14491)
}
