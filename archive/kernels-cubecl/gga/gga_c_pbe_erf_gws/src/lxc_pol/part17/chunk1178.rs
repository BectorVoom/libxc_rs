//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1178/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1178<F: Float>(t2079: F, t3037: F, t858: F, t892: F, t1114: F, t20112: F, t12275: F, t13763: F, t1143: F, t6126: F, t1144: F, t2416: F, t3199: F) -> (F, F, F, F, F, F, F) {
    let t29287 = t2079 * t3037;
    let t29751 = t858 * t892;
    let t29775 = t1114 * t20112;
    let t30098 = t12275 * t13763;
    let t34963 = t1143 * t6126;
    let t35566 = t858 * t1144;
    let t36129 = t3199 * t2416;
    (t29287, t29751, t29775, t30098, t34963, t35566, t36129)
}
