//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 109/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk109<F: Float>(t261: F, t93: F, t108: F, t260: F, t1: F, t183: F, t22: F) -> (F, F, F) {
    let t262 = t93 * t261;
    let t265 = (t260 / F::cast_from(2.0_f64) + t262 / F::cast_from(2.0_f64)) * t108;
    let t266 = t183 * t1;
    let t267 = t266 * t22;
    (t265, t266, t267)
}
