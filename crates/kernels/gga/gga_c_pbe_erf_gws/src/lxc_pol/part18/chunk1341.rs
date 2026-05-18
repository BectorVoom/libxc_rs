//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1341/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1341<F: Float>(t14028: F, t3810: F, t11480: F, t4028: F, t54268: F, t54272: F, t54284: F, t54286: F, t54290: F, t57108: F, t57110: F, t57112: F, t57114: F, t57117: F, t57119: F) -> F {
    let t57121 = t14028 * t3810;
    let t57123 = t4028 * t11480;
    let t57125 = -t57108 / F::new(384.0) - t57110 / F::new(64.0) - F::new(7.0) / F::new(288.0) * t57112 + F::new(3.0) / F::new(256.0) * t57114 - t57117 / F::new(8.0) + t54268 + t57119 / F::new(768.0) - F::new(7.0) / F::new(576.0) * t57121 + t57123 / F::new(48.0) - t54272 + t54284 - t54286 - t54290;
    t57125
}
