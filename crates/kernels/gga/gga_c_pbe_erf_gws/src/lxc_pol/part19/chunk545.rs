//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 545/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk545<F: Float>(t475: F, t987: F, t2858: F, t525: F, t299: F, t991: F, t169: F, t242: F, t171: F, t2522: F, t1086: F, t700: F, t1076: F, t532: F, t1342: F, t1345: F, t1349: F, t1360: F, t1386: F, t1388: F, t1389: F, t145: F, t2848: F) -> (F, F, F, F, F, F, F, F) {
    let t2986 = t475 * t987;
    let t2990 = t525 * t2858;
    let t2994 = t299 * t991;
    let t2996 = t169 * t2994 * t242;
    let t2998 = t171 * t2522;
    let t3003 = t169 * t1086 * t700;
    let t3007 = t532 * t1076;
    let t3011 = -t1342 + 0.53059442957798955452e-1 * t1345 + t1349 + 0.53059442957798955452e-1 * t2996 - 0.31835665774679373271e-1 * t169 * t2998 * t242 - 0.31835665774679373271e-1 * t3003 - 0.31835665774679373271e-1 * t1360 - t1386 + t1388 - 0.1066501354843587606e0 * t1389 - 0.1066501354843587606e0 * t3007 + 0.533250677421793803e-1 * t145 * t2848;
    (t2986, t2990, t2994, t2996, t2998, t3003, t3007, t3011)
}
