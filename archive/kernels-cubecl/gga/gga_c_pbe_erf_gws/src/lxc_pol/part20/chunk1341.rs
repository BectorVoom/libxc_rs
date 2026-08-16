//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1341/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1341<F: Float>(t14028: F, t3810: F, t11480: F, t4028: F, t54268: F, t54272: F, t54284: F, t54286: F, t54290: F, t57108: F, t57110: F, t57112: F, t57114: F, t57117: F, t57119: F) -> F {
    let t57121 = t14028 * t3810;
    let t57123 = t4028 * t11480;
    let t57125 = -t57108 / F::cast_from(384.0_f64) - t57110 / F::cast_from(64.0_f64) - F::cast_from(7.0_f64) / F::cast_from(288.0_f64) * t57112 + F::cast_from(3.0_f64) / F::cast_from(256.0_f64) * t57114 - t57117 / F::cast_from(8.0_f64) + t54268 + t57119 / F::cast_from(768.0_f64) - F::cast_from(7.0_f64) / F::cast_from(576.0_f64) * t57121 + t57123 / F::cast_from(48.0_f64) - t54272 + t54284 - t54286 - t54290;
    t57125
}
