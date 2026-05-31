//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1352/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1352<F: Float>(t51459: F, t54398: F, t54402: F, t57213: F, t57216: F, t57219: F, t57223: F, t57225: F, t57227: F, t57229: F, t57231: F, t57233: F, t57235: F) -> F {
    let t57237 = -t51459 + F::cast_from(7.0_f64) / F::cast_from(576.0_f64) * t57213 + t54398 - t54402 + t57216 / F::cast_from(96.0_f64) - t57219 / F::cast_from(48.0_f64) - t57223 / F::cast_from(96.0_f64) + t57225 / F::cast_from(64.0_f64) + t57227 / F::cast_from(384.0_f64) + t57229 / F::cast_from(48.0_f64) - t57231 / F::cast_from(384.0_f64) + t57233 / F::cast_from(48.0_f64) + F::cast_from(5.0_f64) / F::cast_from(192.0_f64) * t57235;
    t57237
}
