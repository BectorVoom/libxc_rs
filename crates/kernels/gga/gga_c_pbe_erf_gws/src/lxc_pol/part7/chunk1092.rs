//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1092/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1092<F: Float>(t2362: F, t2387: F, t6154: F, t20692: F, t825: F, t20695: F, t829: F, t830: F, t831: F, t4464: F, t6155: F, t2365: F, t4408: F, t822: F, t2373: F, t4453: F) -> (F, F, F, F, F, F) {
    let t21762 = t2387 * t6154 * t2362;
    let t21764 = t20692 * t825;
    let t21768 = t829 * t830 * t831 * t20695;
    let t21771 = t6155 * t4464;
    let t21773 = t4408 * t2365;
    let t21775 = t822 * t21773 * t2362;
    let t21777 = t4453 * t2373;
    (t21762, t21764, t21768, t21771, t21775, t21777)
}
