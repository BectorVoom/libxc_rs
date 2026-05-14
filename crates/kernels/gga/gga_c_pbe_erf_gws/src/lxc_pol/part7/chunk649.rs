//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 649/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk649<F: Float>(t5406: F, t593: F, t1648: F, t1656: F, t1666: F, t1651: F, t1655: F, t587: F, t1923: F, t707: F, t256: F, t1914: F, t1918: F, t247: F, t24: F, t712: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t5408 = 4.0 / 15.0 * t5406 * t593;
    let t5410 = 4.0 / 15.0 * t1648 * t1656;
    let t5412 = 4.0 / 9.0 * t1648 * t1666;
    let t5413 = t1651 * t1655;
    let t5414 = t587 * t5413;
    let t5415 = 8.0 / 45.0 * t5414;
    let t5416 = t707 * t1923;
    let t5417 = t5416 * t256;
    let t5418 = t1914 * t1918;
    let t5420 = t247 * t1923;
    let t5421 = t24 * t5420;
    let t5423 = 0.18233333333333333333e0 * t712 * t5421;
    (t5408, t5410, t5412, t5413, t5415, t5416, t5417, t5418, t5420, t5421, t5423)
}
