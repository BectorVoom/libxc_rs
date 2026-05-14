//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 939/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk939<F: Float>(t2255: F, t3258: F, t6598: F, t254: F, t6: F, t6469: F, t2081: F, t2105: F, t9441: F, t2182: F, t274: F, t1123: F, t2158: F, t3219: F, t3235: F, t2323: F, t3268: F) -> (F, F, F, F, F, F) {
    let t9478 = t2255 * t3258 * t6598;
    let t9482 = t254 * t6 * t6469;
    let t9483 = t2105 * t2081;
    let t9484 = t9441 * t9483;
    let t9485 = t9482 * t9484;
    let t9488 = t274 * t2182;
    let t9490 = t2255 * t1123 * t9488;
    let t9494 = t3235 * t3219 * t2158;
    let t9498 = 7.0 / 576.0 * t2323 * t3268;
    (t9478, t9484, t9485, t9490, t9494, t9498)
}
