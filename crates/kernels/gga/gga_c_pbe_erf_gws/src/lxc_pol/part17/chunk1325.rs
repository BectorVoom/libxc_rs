//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1325/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1325<F: Float>(t4028: F, t9013: F, t1158: F, t51395: F, t14058: F, t3268: F, t1140: F, t14083: F, t3190: F, t3206: F, t2146: F, t14007: F, t9545: F) -> (F, F, F, F, F, F) {
    let t54350 = t4028 * t9013;
    let t54352 = t51395 * t1158;
    let t54354 = t14058 * t3268;
    let t54355 = F::new(7.0) / F::new(288.0) * t54354;
    let t54356 = t14083 * t1140;
    let t54359 = t3206 * t3190;
    let t54360 = t2146 * t54359;
    let t54362 = t14007 * t9545;
    (t54350, t54352, t54355, t54356, t54360, t54362)
}
