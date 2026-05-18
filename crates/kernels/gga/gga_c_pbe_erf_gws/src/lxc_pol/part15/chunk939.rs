//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 939/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk939<F: Float>(t510: F, t967: F, t5651: F, t1083: F, t1473: F, t525: F, t8108: F, t1503: F, t987: F, t1477: F, t991: F, t551: F) -> (F, F, F, F, F) {
    let t8292 = t967 * t510;
    let t8293 = t5651 * t8292;
    let t8296 = t1473 * t1083;
    let t8302 = t525 * t8108;
    let t8305 = t1503 * t987;
    let t8308 = t1477 * t991;
    let t8309 = t8308 * t551;
    (t8293, t8296, t8302, t8305, t8309)
}
