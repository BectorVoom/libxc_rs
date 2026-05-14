//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1147/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1147<F: Float>(t11526: F, t51421: F, t3123: F, t9127: F, t11548: F, t14007: F, t12015: F, t14031: F, t11501: F, t14567: F, t6608: F, t11615: F, t14011: F, t11957: F, t14101: F, t14046: F, t3820: F) -> (F, F, F, F, F, F, F, F) {
    let t56966 = t51421 * t11526;
    let t56968 = t3123 * t9127;
    let t56970 = t14007 * t11548;
    let t56972 = t14031 * t12015;
    let t56975 = t6608 * t11501 * t14567;
    let t56978 = t14011 * t11615;
    let t56980 = t14101 * t11957;
    let t56982 = t14046 * t3820;
    (t56966, t56968, t56970, t56972, t56975, t56978, t56980, t56982)
}
