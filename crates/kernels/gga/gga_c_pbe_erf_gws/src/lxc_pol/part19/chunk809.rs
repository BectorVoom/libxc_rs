//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 809/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk809<F: Float>(t1105: F, t2264: F, t899: F, t923: F, t3249: F, t6636: F, t6684: F, t2323: F, t3279: F, t1158: F, t6505: F, t2344: F, t904: F, t1150: F, t6717: F, t2246: F, t3099: F) -> (F, F, F, F, F, F, F, F) {
    let t9607 = t1105 * param_a_c;
    let t9630 = t899 * t2264 * t923;
    let t9632 = 7.0 / 384.0 * t9630 * t3249;
    let t9637 = t6684 * t6636;
    let t9645 = 35.0 / 576.0 * t2323 * t3279;
    let t9658 = t6505 * t1158;
    let t9665 = t2344 * t904;
    let t9669 = t6717 * t1150;
    let t9695 = 7.0 / 72.0 * t2246 * t3099;
    (t9607, t9632, t9637, t9645, t9658, t9665, t9669, t9695)
}
