//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1349/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1349<F: Float>(t54730: F, t3965: F, t9299: F, t52036: F, t1115: F, t50932: F, t52020: F, t52027: F, t54707: F, t54711: F, t54714: F, t54717: F, t54719: F, t54722: F, t54724: F, t54727: F, t54729: F, t827: F) -> F {
    let t54731 = F::cast_from(7.0_f64) / F::cast_from(1152.0_f64) * t54730;
    let t54734 = t3965 * t9299;
    let t54737 = F::cast_from(35.0_f64) / F::cast_from(216.0_f64) * t52036;
    let t54738 = -t54707 / F::cast_from(768.0_f64) - t827 * t54711 / F::cast_from(48.0_f64) + t54714 / F::cast_from(24.0_f64) + t54717 - F::cast_from(35.0_f64) / F::cast_from(216.0_f64) * t52020 - F::cast_from(35.0_f64) / F::cast_from(216.0_f64) * t54719 - t54722 / F::cast_from(48.0_f64) - F::cast_from(119.0_f64) / F::cast_from(13824.0_f64) * t54724 + t54727 + t54729 + t54731 - t1115 * t50932 / F::cast_from(96.0_f64) - t54734 / F::cast_from(16.0_f64) + F::cast_from(7.0_f64) / F::cast_from(36.0_f64) * t52027 + t54737;
    t54738
}
