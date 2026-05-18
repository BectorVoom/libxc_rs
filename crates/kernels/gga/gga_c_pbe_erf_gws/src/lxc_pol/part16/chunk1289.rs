//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1289/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1289<F: Float>(t1140: F, t14083: F, t3190: F, t3206: F, t2146: F, t14007: F, t9545: F, t9478: F, t14015: F, t9460: F, t14570: F, t6188: F) -> (F, F, F, F, F, F) {
    let t54356 = t14083 * t1140;
    let t54359 = t3206 * t3190;
    let t54360 = t2146 * t54359;
    let t54362 = t14007 * t9545;
    let t54366 = t14007 * t9478;
    let t54368 = t14015 * t9460;
    let t54370 = t6188 * t14570;
    (t54356, t54360, t54362, t54366, t54368, t54370)
}
