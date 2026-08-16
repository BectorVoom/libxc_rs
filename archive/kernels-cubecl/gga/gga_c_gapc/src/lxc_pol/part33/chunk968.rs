//! GGA_C_GAPC lxc pol — lxc_pol part 33 (v4rho2sigma2_12) CSE chunk 968/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part33_v4rho2sigma2_12_chunk968<F: Float>(t11311: F, t916: F, t1086: F, t6188: F, t11320: F, t2636: F, t9554: F, t129: F, t7451: F, t3284: F, t7453: F, t190: F, t277: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t11790 = t916 * t11311;
    let t11791 = t1086 * t6188;
    let t11792 = t11790 * t11791;
    let t11794 = t916 * t11320;
    let t11795 = t2636 * t9554;
    let t11796 = t11794 * t11795;
    let t11798 = t7451 * t129;
    let t11799 = t3284 * t7453;
    let t11800 = t11798 * t11799;
    let t11802 = t277 * t190;
    (t11790, t11791, t11792, t11794, t11795, t11796, t11798, t11799, t11800, t11802)
}
