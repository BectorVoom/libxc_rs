//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1198/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1198<F: Float>(t4029: F, t51407: F, t14083: F, t888: F, t1189: F, t6590: F, t2276: F, t2299: F, t3969: F, t876: F, t9246: F, t2134: F) -> (F, F, F, F, F, F) {
    let t51408 = t51407 * t4029;
    let t51412 = t14083 * t888;
    let t51414 = t1189 * t6590;
    let t51415 = F::new(595.0) / F::new(5184.0) * t51414;
    let t51421 = t2276 * t3969 * t2299;
    let t51430 = t9246 * t876;
    let t51431 = t2134 * t51430;
    (t51408, t51412, t51415, t51421, t51430, t51431)
}
