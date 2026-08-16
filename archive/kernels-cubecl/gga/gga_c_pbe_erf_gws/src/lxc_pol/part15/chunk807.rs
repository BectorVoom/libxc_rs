//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 807/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk807<F: Float>(t2250: F, t6274: F, t810: F, t875: F, t824: F, t745: F, t874: F, t343: F, t2189: F, t274: F, t2145: F, t2387: F) -> (F, F, F, F, F, F) {
    let t6275 = t2250 * t6274;
    let t6277 = t875 * t810;
    let t6278 = t824 * t6277;
    let t6296 = t745 * t874;
    let t6297 = t6296 * t343;
    let t6303 = t274 * t2189 * t343;
    let t6322 = t2387 * t2145;
    (t6275, t6277, t6278, t6297, t6303, t6322)
}
