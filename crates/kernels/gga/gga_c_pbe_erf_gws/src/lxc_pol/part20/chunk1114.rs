//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1114/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1114<F: Float>(t2201: F, t326: F, t378: F, t4009: F, t4414: F, t13952: F, t886: F) -> (F, F, F, F) {
    let t13987 = t326 * t2201;
    let t13988 = t13987 * t378;
    let t13989 = F::cast_from(35.0_f64) / F::cast_from(432.0_f64) * t13988;
    let t13999 = t4414 * t4009;
    let t14001 = t13952 * t886;
    (t13987, t13989, t13999, t14001)
}
