//! GGA_C_GAPC lxc pol — lxc_pol part 29 (v4rho2sigma2_8) CSE chunk 988/1311 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part29_v4rho2sigma2_8_chunk988<F: Float>(t11937: F, t11938: F, t3439: F, t772: F, t3438: F, t11379: F, t9894: F, t829: F, t9896: F, t3402: F, t3708: F, t9934: F) -> (F, F, F, F, F, F, F, F) {
    let t11939 = t11937 * t11938;
    let t11941 = t772 * t3439;
    let t11942 = t3438 * t11941;
    let t11944 = t9894 * t11379;
    let t11945 = t829 * t9896;
    let t11946 = t11944 * t11945;
    let t11948 = t3402 * t3708;
    let t11949 = t11948 * t9934;
    (t11939, t11941, t11942, t11944, t11945, t11946, t11948, t11949)
}
