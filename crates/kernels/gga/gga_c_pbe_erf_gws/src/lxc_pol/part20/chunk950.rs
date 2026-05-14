//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 950/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk950<F: Float>(t11860: F, t2168: F, t6481: F, t2319: F, t3810: F, t3128: F, t8963: F, t3792: F, t6183: F, t3116: F, t11844: F, t11846: F, t11849: F, t11852: F, t11854: F, t11857: F, t2253: F, t6456: F, t9539: F) -> (F, F, F, F, F) {
    let t11862 = t2168 * t11860 / 48.0;
    let t11863 = 35.0 / 216.0 * t6481;
    let t11864 = t2319 * t3810;
    let t11867 = t3128 * t8963 / 24.0;
    let t11868 = t6183 * t3792;
    let t11869 = t3116 * t11868;
    let t11870 = 7.0 / 288.0 * t11869;
    let t11871 = t11844 - 119.0 / 6912.0 * t6456 - 7.0 / 768.0 * t11846 - t2253 * t11849 / 768.0 + 7.0 / 576.0 * t11852 - t2253 * t11854 / 768.0 + t9539 - 7.0 / 288.0 * t11857 + t11862 - t11863 - 7.0 / 1152.0 * t11864 - t11867 + t11870;
    (t11862, t11863, t11867, t11870, t11871)
}
