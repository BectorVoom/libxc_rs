//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1349/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1349<F: Float>(t54730: F, t3965: F, t9299: F, t52036: F, t1115: F, t50932: F, t52020: F, t52027: F, t54707: F, t54711: F, t54714: F, t54717: F, t54719: F, t54722: F, t54724: F, t54727: F, t54729: F, t827: F) -> F {
    let t54731 = F::new(7.0) / F::new(1152.0) * t54730;
    let t54734 = t3965 * t9299;
    let t54737 = F::new(35.0) / F::new(216.0) * t52036;
    let t54738 = -t54707 / F::new(768.0) - t827 * t54711 / F::new(48.0) + t54714 / F::new(24.0) + t54717 - F::new(35.0) / F::new(216.0) * t52020 - F::new(35.0) / F::new(216.0) * t54719 - t54722 / F::new(48.0) - F::new(119.0) / F::new(13824.0) * t54724 + t54727 + t54729 + t54731 - t1115 * t50932 / F::new(96.0) - t54734 / F::new(16.0) + F::new(7.0) / F::new(36.0) * t52027 + t54737;
    t54738
}
