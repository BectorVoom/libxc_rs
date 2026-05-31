//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 383/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk383<F: Float>(t11: F, t19: F, t1237: F, t1240: F, t398: F, t21: F, t703: F) -> (F, F, F, F) {
    let t1245 = F::cast_from(1.0_f64)/F::sqrt(t11);
    let t1246 = t1245 * t19;
    let t1247 = t1246 * t1237;
    let t1249 = t398 * t1240;
    let t1251 = t21 * t703;
    (t1246, t1247, t1249, t1251)
}
