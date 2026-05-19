//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1000/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1000<F: Float>(t17346: F, t17348: F, t17354: F, t17359: F, t17362: F, t17364: F, t17368: F, t18256: F, t18261: F, t18267: F, t18268: F, t17372: F, t17376: F, t17378: F, t17382: F, t17384: F, t17386: F, t17391: F, t17394: F, t17397: F, t17402: F, t17404: F, t17406: F) -> (F, F) {
    let t18270 = F::cast_from(0.72933333333333333331e0_f64) * t18256 + t18261 + t18267 + F::new(8.0) / F::new(3.0) * t18268 + t17346 - t17348 + t17354 - t17359 - t17362 + t17364 - t17368;
    let t18271 = t17372 - t17376 + t17378 - t17382 - t17384 - t17386 + t17391 + t17394 + t17397 + t17402 - t17404 + t17406;
    (t18270, t18271)
}
