//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1167/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1167<F: Float>(t2212: F, t6480: F, t2170: F, t332: F, t2122: F, t6277: F, t6678: F, t2332: F, t899: F, t912: F, t2348: F, t336: F, t9239: F) -> (F, F, F, F) {
    let t20831 = t6480 * t2212;
    let t20832 = F::cast_from(35.0_f64) / F::cast_from(12.0_f64) * t20831;
    let t20833 = t332 * t2170;
    let t20835 = t20833 * t2122 * t6277;
    let t20837 = t6678 * t20835 / F::cast_from(4.0_f64);
    let t20839 = t899 * t912 * t2332;
    let t20840 = t20839 * t2348;
    let t20842 = t9239 * t336;
    (t20832, t20837, t20840, t20842)
}
