//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1328/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1328<F: Float>(t54377: F, t4171: F, t51407: F, t4049: F, t9661: F, t4043: F, t9449: F, t51437: F, t51439: F, t51447: F, t51452: F, t54366: F, t54368: F, t54370: F, t54374: F) -> F {
    let t54378 = F::new(7.0) / F::new(72.0) * t54377;
    let t54381 = t51407 * t4171;
    let t54384 = t4049 * t9661;
    let t54386 = t4043 * t9449;
    let t54388 = -t54366 / F::new(384.0) - t54368 / F::new(96.0) - t54370 / F::new(96.0) + t54374 / F::new(48.0) + F::new(7.0) / F::new(288.0) * t51437 + t54378 + F::new(7.0) / F::new(144.0) * t51439 + F::new(7.0) / F::new(576.0) * t51447 - F::new(35.0) / F::new(432.0) * t54381 + F::new(7.0) / F::new(1152.0) * t51452 - t54384 / F::new(384.0) - t54386 / F::new(768.0);
    t54388
}
