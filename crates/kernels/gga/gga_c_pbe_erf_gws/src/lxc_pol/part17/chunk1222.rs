//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1222/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1222<F: Float>(t14387: F, t804: F, t1198: F, t2429: F, t6926: F, t13917: F, t4149: F, t9521: F, t14765: F, t2118: F, t3074: F, t6778: F) -> (F, F, F, F) {
    let t52884 = F::cast_from(6.0_f64) * t804 * t14387;
    let t52887 = F::cast_from(12.0_f64) * t2429 * t1198 * t6926;
    let t52889 = t13917 * t4149 * t9521;
    let t52893 = t3074 * t2118 * t14765 * t6778;
    (t52884, t52887, t52889, t52893)
}
