//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1232/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1232<F: Float>(t13930: F, t26958: F, t14402: F, t4386: F, t892: F, t50998: F, t51066: F, t9650: F, t1105: F, t353: F, t4053: F, t1193: F, t2494: F) -> (F, F, F, F, F) {
    let t53028 = F::cast_from(7.0_f64) / F::cast_from(72.0_f64) * t26958 * t13930;
    let t53034 = t4386 * t892 * t14402;
    let t53038 = t50998 * t51066 * t9650;
    let t53042 = t4386 * t353 * t4053 * t1105;
    let t53047 = t4386 * t353 * t1193 * t2494;
    (t53028, t53034, t53038, t53042, t53047)
}
