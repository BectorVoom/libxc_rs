//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1184/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1184<F: Float>(t50935: F, t822: F, t824: F, t838: F, t13984: F, t2331: F, t328: F, t356: F, t3971: F, t3976: F, t15636: F, t3973: F) -> (F, F, F, F, F, F, F) {
    let t50936 = t822 * t50935;
    let t50942 = t824 * t838;
    let t50943 = t822 * t50942;
    let t50944 = t50943 * t13984;
    let t50948 = t356 * t328 * t2331 * t3971;
    let t50949 = t50948 * t3976;
    let t50956 = t3973 * t15636;
    (t50936, t50942, t50943, t50944, t50948, t50949, t50956)
}
