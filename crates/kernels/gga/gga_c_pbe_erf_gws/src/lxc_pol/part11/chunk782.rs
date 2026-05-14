//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 782/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk782<F: Float>(t11668: F, t3793: F, t11581: F, t11598: F, t13349: F, t13355: F, t13357: F, t13361: F, t13363: F, t13367: F, t13373: F, t13377: F, t13379: F, t13384: F, t13387: F, t2277: F, t2312: F, t2343: F) -> (F, F) {
    let t13391 = t11668 * t3793 / 48.0;
    let t13392 = t2343 * t13349 / 128.0 + t13355 - t2312 * t13357 / 128.0 - t13361 - t2277 * t13363 / 256.0 - t13367 + t13373 + t13377 - t2343 * t13379 / 512.0 - 7.0 / 768.0 * t11581 + 7.0 / 96.0 * t11598 - t13384 - t2312 * t13387 / 128.0 - t13391;
    (t13391, t13392)
}
