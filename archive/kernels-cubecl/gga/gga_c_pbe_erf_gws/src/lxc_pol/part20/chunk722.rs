//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 722/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk722<F: Float>(t4043: F, t918: F, t1189: F, t925: F, t366: F, t864: F, t899: F) -> (F, F, F) {
    let t4044 = t4043 * t918;
    let t4046 = t1189 * t925;
    let t4047 = F::cast_from(7.0_f64) / F::cast_from(2304.0_f64) * t4046;
    let t4049 = t899 * t864 * t366;
    (t4044, t4047, t4049)
}
