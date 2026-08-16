//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 813/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk813<F: Float>(t378: F, t6729: F, t6182: F, t6186: F, t6190: F, t6219: F, t6224: F, t6230: F, t6246: F, t6251: F, t6255: F, t6260: F, t6273: F, t6321: F, t6324: F) -> (F, F) {
    let t6731 = F::cast_from(455.0_f64) / F::cast_from(1296.0_f64) * t6729 * t378;
    let t6732 = -t6182 + t6186 - t6190 - t6219 + t6224 - t6230 - t6246 + t6251 - t6255 - t6260 + t6273 - t6321 - t6324;
    (t6731, t6732)
}
