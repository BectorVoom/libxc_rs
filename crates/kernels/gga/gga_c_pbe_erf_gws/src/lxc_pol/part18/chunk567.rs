//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 567/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk567<F: Float>(t475: F, t987: F, t2858: F, t525: F, t299: F, t991: F, t169: F, t242: F, t171: F, t2522: F, t1086: F, t700: F) -> (F, F, F, F, F, F) {
    let t2986 = t475 * t987;
    let t2990 = t525 * t2858;
    let t2994 = t299 * t991;
    let t2996 = t169 * t2994 * t242;
    let t2998 = t171 * t2522;
    let t3003 = t169 * t1086 * t700;
    (t2986, t2990, t2994, t2996, t2998, t3003)
}
