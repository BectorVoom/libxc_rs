//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 904/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk904<F: Float>(t17372: F, t17376: F, t17378: F, t17382: F, t17384: F, t17386: F, t17391: F, t17394: F, t17397: F, t17402: F, t17404: F, t17406: F, t1617: F, t1841: F, t5519: F, t732: F) -> (F, F, F) {
    let t18271 = t17372 - t17376 + t17378 - t17382 - t17384 - t17386 + t17391 + t17394 + t17397 + t17402 - t17404 + t17406;
    let t18274 = t1841 * t1617;
    let t18276 = t732 * t5519;
    (t18271, t18274, t18276)
}
