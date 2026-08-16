//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 965/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk965<F: Float>(t1882: F, t5379: F, t1627: F, t5138: F, t1661: F, t1802: F, t5480: F, t649: F, t5523: F, t639: F, t4934: F, t5038: F) -> (F, F, F, F, F) {
    let t17846 = t5379 * t1882;
    let t17850 = t1627 * t5138;
    let t17852 = t1661 * t1802;
    let t17870 = t5480 * t649;
    let t17872 = t639 * t17870 * t5523;
    let t17875 = t639 * t4934 * t5038;
    (t17846, t17850, t17852, t17872, t17875)
}
