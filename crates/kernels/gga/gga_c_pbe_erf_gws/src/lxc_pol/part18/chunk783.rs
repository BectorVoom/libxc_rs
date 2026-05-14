//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 783/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk783<F: Float>(t285: F, t3013: F, t545: F, t39: F, t991: F, t159: F, t2522: F, t532: F, t510: F, t967: F, t5651: F, t1083: F, t1473: F, t525: F, t8108: F, t1477: F) -> (F, F, F, F, F, F, F) {
    let t8277 = t3013 * t545 * t285;
    let t8279 = t39 * t991;
    let t8281 = t8279 * t159 * t285;
    let t8287 = t532 * t2522;
    let t8290 = 0.58113483035773838734e-3 * t8287 * t159 * t285;
    let t8292 = t967 * t510;
    let t8293 = t5651 * t8292;
    let t8296 = t1473 * t1083;
    let t8302 = t525 * t8108;
    let t8308 = t1477 * t991;
    (t8277, t8281, t8290, t8293, t8296, t8302, t8308)
}
