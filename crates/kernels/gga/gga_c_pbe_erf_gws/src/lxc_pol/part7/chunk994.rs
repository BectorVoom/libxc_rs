//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 994/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk994<F: Float>(t2379: F, t4453: F, t2246: F, t4446: F, t2242: F, t2420: F, t2271: F, t4422: F, t822: F, t833: F, t4414: F, t6140: F, t6729: F, t941: F, t2352: F, t6726: F, t840: F) -> (F, F, F, F, F, F, F, F) {
    let t19999 = t4453 * t2379;
    let t20007 = t2246 * t4446;
    let t20009 = t2242 * t2420;
    let t20015 = t2271 * t4422;
    let t20017 = t822 * t20015 * t833;
    let t20024 = t4414 * t6140;
    let t20026 = t6729 * t941;
    let t20028 = t2352 * t2352;
    let t20034 = t840 * t6726;
    (t19999, t20007, t20009, t20017, t20024, t20026, t20028, t20034)
}
