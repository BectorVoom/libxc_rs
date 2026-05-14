//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 461/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk461<F: Float>(t2970: F, t672: F, t475: F, t987: F, t2858: F, t525: F, t299: F, t991: F, t169: F, t242: F, t1086: F, t700: F, t1076: F, t532: F) -> (F, F, F, F, F, F, F, F) {
    let t2971 = t2970 * t672;
    let t2986 = t475 * t987;
    let t2990 = t525 * t2858;
    let t2994 = t299 * t991;
    let t2996 = t169 * t2994 * t242;
    let t3003 = t169 * t1086 * t700;
    let t3007 = t532 * t1076;
    let t3013 = t532 * t991;
    (t2971, t2986, t2990, t2994, t2996, t3003, t3007, t3013)
}
