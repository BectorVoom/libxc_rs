//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 766/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk766<F: Float>(t7278: F, t2701: F, t395: F, t2715: F, t401: F, t2712: F, t2775: F, t2778: F, t1001: F, t1243: F, t2769: F, t7271: F, t2762: F, t2765: F, t1014: F, t1251: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t7279 = 0.15996296296296296296e-1 * t7278;
    let t7280 = t395 * t2701;
    let t7288 = 0.17777777777777777778e-1 * t401 * t2715;
    let t7290 = 0.2962962962962962963e-2 * t401 * t2712;
    let t7335 = 0.2962962962962962963e-2 * t401 * t2775;
    let t7364 = 0.17777777777777777778e-1 * t401 * t2778;
    let t7374 = t1243 * t1001;
    let t7376 = t7271 * t2769;
    let t7378 = t395 * t2762;
    let t7379 = 0.15996296296296296296e-1 * t7378;
    let t7380 = t395 * t2765;
    let t7407 = t1251 * t1014;
    (t7279, t7280, t7288, t7290, t7335, t7364, t7374, t7376, t7378, t7379, t7380, t7407)
}
