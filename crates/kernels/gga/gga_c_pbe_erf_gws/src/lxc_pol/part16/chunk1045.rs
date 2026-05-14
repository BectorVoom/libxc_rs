//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1045/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1045<F: Float>(t27691: F, t328: F, t2118: F, t3074: F, t12275: F, t13763: F, t1143: F, t6126: F, t1144: F, t858: F, t2416: F, t3199: F, t326: F, t825: F, t6148: F, t3067: F, t830: F) -> (F, F, F, F, F, F, F, F) {
    let t29843 = t27691 * t328;
    let t29845 = t3074 * t2118 * t29843;
    let t30098 = t12275 * t13763;
    let t34963 = t1143 * t6126;
    let t35566 = t858 * t1144;
    let t36129 = t3199 * t2416;
    let t36199 = t326 * t825;
    let t36200 = t36199 * t6148;
    let t36201 = t830 * t3067;
    (t29845, t30098, t34963, t35566, t36129, t36199, t36200, t36201)
}
