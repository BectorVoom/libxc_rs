//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1158/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1158<F: Float>(t14570: F, t6188: F, t2407: F, t26623: F, t858: F, t2120: F, t3195: F, t4033: F, t4171: F, t51407: F, t4049: F, t9661: F, t4043: F, t9449: F, t51437: F, t51439: F, t51447: F, t51452: F, t54366: F, t54368: F) -> (F,) {
    let t54370 = t6188 * t14570;
    let t54373 = t2407 * t858 * t26623;
    let t54374 = t2120 * t54373;
    let t54377 = t4033 * t3195;
    let t54378 = 7.0 / 72.0 * t54377;
    let t54381 = t51407 * t4171;
    let t54384 = t4049 * t9661;
    let t54386 = t4043 * t9449;
    let t54388 = -t54366 / 384.0 - t54368 / 96.0 - t54370 / 96.0 + t54374 / 48.0 + 7.0 / 288.0 * t51437 + t54378 + 7.0 / 144.0 * t51439 + 7.0 / 576.0 * t51447 - 35.0 / 432.0 * t54381 + 7.0 / 1152.0 * t51452 - t54384 / 384.0 - t54386 / 768.0;
    (t54388,)
}
