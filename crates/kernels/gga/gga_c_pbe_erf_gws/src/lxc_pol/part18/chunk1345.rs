//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1345/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1345<F: Float>(t11680: F, t14015: F, t11685: F, t11949: F, t14007: F, t51408: F, t54320: F, t54323: F, t57151: F, t57154: F, t57156: F, t57158: F, t57160: F, t57162: F) -> F {
    let t57164 = t14015 * t11680;
    let t57166 = t14015 * t11685;
    let t57168 = t14007 * t11949;
    let t57170 = t57151 / F::new(192.0) - t54320 - F::new(35.0) / F::new(432.0) * t51408 + t57154 / F::new(48.0) - t54323 - t57156 / F::new(48.0) - t57158 / F::new(96.0) + F::new(7.0) / F::new(144.0) * t57160 - t57162 / F::new(96.0) - t57164 / F::new(96.0) - t57166 / F::new(96.0) - t57168 / F::new(768.0);
    t57170
}
