//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 862/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk862<F: Float>(t1001: F, t1243: F, t2769: F, t7271: F, t2762: F, t395: F, t2765: F, t4949: F, t7341: F, t11: F, t1758: F, t7326: F) -> (F, F, F, F, F, F, F) {
    let t7374 = t1243 * t1001;
    let t7376 = t7271 * t2769;
    let t7378 = t395 * t2762;
    let t7379 = F::new(0.15996296296296296296e-1) * t7378;
    let t7380 = t395 * t2765;
    let t7382 = t4949 * t7341;
    let t7383 = t11 * t7382;
    let t7385 = t1758 * t7326;
    (t7374, t7376, t7378, t7379, t7380, t7383, t7385)
}
