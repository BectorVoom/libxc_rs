//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1149/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1149<F: Float>(t12025: F, t51421: F, t11996: F, t14007: F, t11455: F, t14092: F, t14538: F, t11652: F, t14498: F, t14064: F, t3783: F, t11820: F, t14011: F, t11758: F, t4039: F, t11531: F, t14015: F) -> (F, F, F, F, F, F, F, F) {
    let t57004 = t51421 * t12025;
    let t57006 = t14007 * t11996;
    let t57009 = t14538 * t14092 * t11455;
    let t57011 = t14498 * t11652;
    let t57013 = t3783 * t14064;
    let t57015 = t14011 * t11820;
    let t57017 = t4039 * t11758;
    let t57019 = t14015 * t11531;
    (t57004, t57006, t57009, t57011, t57013, t57015, t57017, t57019)
}
