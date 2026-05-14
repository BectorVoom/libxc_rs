//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 736/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk736<F: Float>(t2281: F, t6416: F, t2100: F, t369: F, t814: F, t931: F, t2298: F, t322: F, t339: F, t6385: F, t2074: F, t871: F, t4379: F, t2178: F, t2181: F, t2183: F, t2186: F, t340: F, t6084: F, t870: F) -> (F, F, F, F, F, F, F, F) {
    let t6417 = t6416 * t2281;
    let t6421 = t2100 * t369;
    let t6424 = t814 * t931;
    let t6429 = t322 * t2298;
    let t6430 = t339 * t6385;
    let t6433 = t871 * t2074;
    let t6436 = t339 * t4379;
    let t6439 = -t339 * t340 * t6084 + 9.0 * t2178 * t2186 - 36.0 * t2181 * t6433 - 36.0 * t2183 * t6424 + 9.0 * t6421 * t871 + 60.0 * t6429 * t6430 + 3.0 * t6436 * t870;
    (t6417, t6421, t6424, t6429, t6430, t6433, t6436, t6439)
}
