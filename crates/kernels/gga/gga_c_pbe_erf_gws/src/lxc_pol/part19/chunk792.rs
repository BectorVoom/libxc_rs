//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 792/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk792<F: Float>(t1118: F, t814: F, t353: F, t4386: F, t2501: F, t810: F, t2370: F, t830: F, t1105: F, t898: F, t938: F, t2416: F, t891: F, t2367: F, t2503: F, t1114: F, t6744: F) -> (F, F, F, F, F, F, F, F) {
    let t8698 = t1118 * t814;
    let t8699 = t353 * t8698;
    let t8700 = t4386 * t8699;
    let t8708 = t2501 * t810;
    let t8710 = t2370 * t830 * t8708;
    let t8713 = t898 * t1105;
    let t8714 = t8713 * t938;
    let t8715 = t353 * t8714;
    let t8716 = t4386 * t8715;
    let t8734 = t891 * t2416;
    let t8740 = 7.0 / 144.0 * t2367 * t2503;
    let t8743 = t1114 * t6744;
    (t8700, t8708, t8710, t8713, t8716, t8734, t8740, t8743)
}
