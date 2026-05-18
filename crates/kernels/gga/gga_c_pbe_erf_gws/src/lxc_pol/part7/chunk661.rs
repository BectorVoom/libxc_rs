//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 661/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk661<F: Float>(t1860: F, t401: F, t1856: F, t4958: F, t4963: F, t1251: F, t607: F, t1863: F, t1857: F, t177: F, t572: F, t191: F) -> (F, F, F, F, F, F, F) {
    let t5248 = t401 * t1860;
    let t5250 = t1856 * t4958;
    let t5253 = t1856 * t4963;
    let t5256 = t1251 * t607;
    let t5258 = t401 * t1863;
    let t5260 = t401 * t1857;
    let t5263 = F::new(1.0) / t177 / t572;
    let t5264 = t191 * t5263;
    (t5248, t5250, t5253, t5256, t5258, t5260, t5264)
}
