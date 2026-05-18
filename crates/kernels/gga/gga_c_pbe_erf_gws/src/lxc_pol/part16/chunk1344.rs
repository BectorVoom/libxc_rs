//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1344/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1344<F: Float>(t54377: F, t54381: F, t51437: F, t51439: F, t51447: F, t51452: F, t54366: F, t54368: F, t54370: F, t54374: F, t54384: F, t54386: F) -> F {
    let t55620 = F::new(7.0) / F::new(36.0) * t54377;
    let t55623 = F::new(35.0) / F::new(216.0) * t54381;
    let t55627 = -t54366 / F::new(192.0) - t54368 / F::new(48.0) - t54370 / F::new(48.0) + t54374 / F::new(24.0) + F::new(7.0) / F::new(144.0) * t51437 + t55620 + F::new(7.0) / F::new(72.0) * t51439 + F::new(7.0) / F::new(288.0) * t51447 - t55623 + F::new(7.0) / F::new(576.0) * t51452 - t54384 / F::new(192.0) - t54386 / F::new(384.0);
    t55627
}
