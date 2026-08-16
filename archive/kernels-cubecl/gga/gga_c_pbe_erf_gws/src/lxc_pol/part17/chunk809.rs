//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 809/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk809<F: Float>(t1: F, t6383: F, t2313: F, t814: F, t2156: F, t274: F, t343: F, t6201: F, t915: F, t2250: F, t2259: F, t6269: F) -> (F, F, F, F, F, F, F, F) {
    let t6384 = t6383 * t1;
    let t6390 = t2313 * t814;
    let t6395 = t274 * t2156;
    let t6396 = t6395 * t343;
    let t6401 = t6201 * t915;
    let t6402 = t2250 * t6401;
    let t6403 = t6402 * t2259;
    let t6409 = t6269 * t343;
    (t6384, t6390, t6395, t6396, t6401, t6402, t6403, t6409)
}
