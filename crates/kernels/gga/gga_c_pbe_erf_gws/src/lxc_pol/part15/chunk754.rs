//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 754/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk754<F: Float>(t2157: F, t343: F, t2306: F, t346: F, t2382: F, t2074: F, t337: F, t5: F, t2147: F, t2189: F, t2251: F, t933: F, t2250: F, t810: F, t875: F, t824: F) -> (F, F, F, F, F, F, F, F) {
    let t6241 = t2157 * t343;
    let t6252 = t2306 * t346;
    let t6253 = t2382 * t6252;
    let t6257 = t337 * t5 * t2074;
    let t6258 = t2147 * t6257;
    let t6269 = t5 * t2189;
    let t6274 = t2251 * t933;
    let t6275 = t2250 * t6274;
    let t6277 = t875 * t810;
    let t6278 = t824 * t6277;
    (t6241, t6253, t6257, t6258, t6269, t6275, t6277, t6278)
}
