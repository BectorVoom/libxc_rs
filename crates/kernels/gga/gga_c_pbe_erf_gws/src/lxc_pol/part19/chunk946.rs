//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 946/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk946<F: Float>(t11811: F, t11363: F, t6659: F, t858: F, t884: F, t2142: F, t3783: F, t1134: F, t3189: F, t9343: F, t2255: F, t3111: F, t3752: F, t3037: F, t816: F, t3257: F, t3258: F) -> (F, F, F, F, F, F, F) {
    let t11812 = 7.0 / 288.0 * t11811;
    let t11814 = t6659 * t858 * t11363;
    let t11816 = t884 * t11814 / 4.0;
    let t11817 = t3783 * t2142;
    let t11818 = 7.0 / 288.0 * t11817;
    let t11819 = t1134 * t3189;
    let t11820 = t9343 * t11819;
    let t11824 = t2255 * t3111 * t3752;
    let t11827 = t816 * t3037;
    let t11829 = t3257 * t3258 * t11827;
    (t11812, t11816, t11818, t11819, t11820, t11824, t11829)
}
