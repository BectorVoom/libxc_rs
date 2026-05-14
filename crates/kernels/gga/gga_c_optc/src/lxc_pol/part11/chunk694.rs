//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 694/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk694<F: Float>(t3881: F, t8193: F, t1: F, t935: F, t297: F, t123: F, t2672: F, t2747: F, t282: F, t7380: F, t3916: F, t1781: F, t287: F, t321: F, t320: F, t92: F, sigma0: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t8194 = t3881 * t8193;
    let t8196 = t935 * t1;
    let t8197 = t8196 * t297;
    let t8201 = t2672 * t123;
    let t8206 = t2747 * sigma0;
    let t8207 = t8206 * t282;
    let t8208 = t8207 * t8193;
    let t8209 = t7380 * t935;
    let t8210 = t8209 * t1;
    let t8214 = t3916 * t8193;
    let t8215 = t2672 * t935;
    let t8216 = t8215 * t1;
    let t8229 = t321 * t1781 * t287;
    let t8231 = 0.32196894406625029092e-1 * t320 * t8229;
    let t8285 = t92 * t92;
    (t8194, t8196, t8197, t8201, t8206, t8207, t8208, t8209, t8210, t8214, t8215, t8216, t8231, t8285)
}
