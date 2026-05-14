//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 927/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk927<F: Float>(t11478: F, t2170: F, t2171: F, t2168: F, t3180: F, t9188: F, t8956: F, t3793: F, t8949: F, t11451: F, t11458: F, t11463: F, t11466: F, t11472: F, t11477: F, t2253: F, t2343: F, t8826: F, t8835: F, t8846: F) -> (F, F, F, F, F, F) {
    let t11480 = t2170 * t11478 * t2171;
    let t11482 = t2168 * t11480 / 48.0;
    let t11484 = t9188 * t3180 / 24.0;
    let t11486 = t8956 * t3180 / 24.0;
    let t11488 = t8949 * t3793 / 96.0;
    let t11489 = t8826 - t2253 * t11451 / 384.0 + t8835 - t11458 + t11463 + t2343 * t11466 / 384.0 - t11472 + t11477 + t11482 - t11484 - t11486 - t8846 - t11488;
    (t11480, t11482, t11484, t11486, t11488, t11489)
}
