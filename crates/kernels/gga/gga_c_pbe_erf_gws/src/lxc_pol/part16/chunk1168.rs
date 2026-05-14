//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1168/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1168<F: Float>(t51431: F, t54338: F, t54342: F, t54346: F, t54348: F, t54350: F, t54360: F, t54362: F, t55603: F, t55607: F, t55608: F, t55609: F, t54377: F, t54381: F, t51437: F, t51439: F, t51447: F, t51452: F, t54366: F, t54368: F, t54370: F, t54374: F, t54384: F, t54386: F) -> (F, F) {
    let t55613 = -5.0 / 48.0 * t54338 + t54342 / 24.0 - t55603 - 5.0 / 32.0 * t54346 - t54348 / 24.0 - t54350 / 48.0 - t55607 + t55608 - t55609 + 7.0 / 72.0 * t51431 + t54360 / 4.0 + t54362 / 192.0;
    let t55620 = 7.0 / 36.0 * t54377;
    let t55623 = 35.0 / 216.0 * t54381;
    let t55627 = -t54366 / 192.0 - t54368 / 48.0 - t54370 / 48.0 + t54374 / 24.0 + 7.0 / 144.0 * t51437 + t55620 + 7.0 / 72.0 * t51439 + 7.0 / 288.0 * t51447 - t55623 + 7.0 / 576.0 * t51452 - t54384 / 192.0 - t54386 / 384.0;
    (t55613, t55627)
}
