//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 812/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk812<F: Float>(t297: F, t8196: F, t8195: F, t123: F, t2672: F, t2606: F, t8185: F, t2747: F, t282: F, t8193: F, t7380: F, t935: F, t1: F, t3916: F, t7885: F, t952: F, sigma0: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t8197 = t8196 * t297;
    let t8198 = t8195 * t8197;
    let t8201 = t2672 * t123;
    let t8202 = t8201 * t2606;
    let t8203 = t8185 * t8202;
    let t8206 = t2747 * sigma0;
    let t8207 = t8206 * t282;
    let t8208 = t8207 * t8193;
    let t8209 = t7380 * t935;
    let t8210 = t8209 * t1;
    let t8211 = t8195 * t8210;
    let t8214 = t3916 * t8193;
    let t8215 = t2672 * t935;
    let t8216 = t8215 * t1;
    let t8217 = t8195 * t8216;
    let t8220 = t952 * t7885;
    (t8197, t8198, t8201, t8203, t8206, t8207, t8208, t8209, t8210, t8211, t8214, t8215, t8216, t8217, t8220)
}
