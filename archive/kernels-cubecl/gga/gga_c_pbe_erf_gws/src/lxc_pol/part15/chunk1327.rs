//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1327/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1327<F: Float>(t14007: F, t9478: F, t14015: F, t9460: F, t14570: F, t6188: F, t2407: F, t26623: F, t858: F, t2120: F, t3195: F, t4033: F) -> (F, F, F, F, F) {
    let t54366 = t14007 * t9478;
    let t54368 = t14015 * t9460;
    let t54370 = t6188 * t14570;
    let t54373 = t2407 * t858 * t26623;
    let t54374 = t2120 * t54373;
    let t54377 = t4033 * t3195;
    (t54366, t54368, t54370, t54374, t54377)
}
