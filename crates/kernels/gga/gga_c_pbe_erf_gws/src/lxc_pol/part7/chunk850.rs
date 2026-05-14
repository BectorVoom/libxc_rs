//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 850/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk850<F: Float>(t16986: F, t1891: F, t639: F, t642: F, t1648: F, t5510: F, t1820: F, t1866: F, t1885: F, t5307: F, t17339: F, t1888: F, t5015: F, t5312: F, t17354: F, t17359: F, t17362: F, t17364: F, t17368: F, t17372: F) -> (F, F, F, F, F, F) {
    let t17376 = 8.0 / 15.0 * t639 * t642 * t1891 * t16986;
    let t17378 = 32.0 / 15.0 * t1648 * t5510;
    let t17382 = 8.0 / 5.0 * t1820 * t1885 * t5307 * t1866;
    let t17384 = 16.0 / 5.0 * t17339 * t1888;
    let t17386 = 16.0 / 5.0 * t5312 * t5015;
    let t17387 = t17354 - t17359 - t17362 + t17364 - t17368 + t17372 - t17376 + t17378 - t17382 - t17384 - t17386;
    (t17376, t17378, t17382, t17384, t17386, t17387)
}
