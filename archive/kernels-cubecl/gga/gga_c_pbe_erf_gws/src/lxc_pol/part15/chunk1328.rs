//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1328/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1328<F: Float>(t54377: F, t4171: F, t51407: F, t4049: F, t9661: F, t4043: F, t9449: F, t51437: F, t51439: F, t51447: F, t51452: F, t54366: F, t54368: F, t54370: F, t54374: F) -> F {
    let t54378 = F::cast_from(7.0_f64) / F::cast_from(72.0_f64) * t54377;
    let t54381 = t51407 * t4171;
    let t54384 = t4049 * t9661;
    let t54386 = t4043 * t9449;
    let t54388 = -t54366 / F::cast_from(384.0_f64) - t54368 / F::cast_from(96.0_f64) - t54370 / F::cast_from(96.0_f64) + t54374 / F::cast_from(48.0_f64) + F::cast_from(7.0_f64) / F::cast_from(288.0_f64) * t51437 + t54378 + F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t51439 + F::cast_from(7.0_f64) / F::cast_from(576.0_f64) * t51447 - F::cast_from(35.0_f64) / F::cast_from(432.0_f64) * t54381 + F::cast_from(7.0_f64) / F::cast_from(1152.0_f64) * t51452 - t54384 / F::cast_from(384.0_f64) - t54386 / F::cast_from(768.0_f64);
    t54388
}
