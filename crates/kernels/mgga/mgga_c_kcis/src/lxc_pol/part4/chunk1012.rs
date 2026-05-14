//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1012/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1012<F: Float>(t1114: F, t13786: F, t345: F, t2952: F, t4601: F, t4600: F, t313: F, t4625: F, t1045: F, t3293: F, t1728: F, t3096: F, t4642: F, t4637: F, t4852: F, t1762: F, t3251: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t14274 = t1114 * t13786;
    let t14275 = t345 * t14274;
    let t14278 = t4601 * t2952;
    let t14279 = t4600 * t14278;
    let t14282 = t313 * t4625;
    let t14283 = t14282 * t1045;
    let t14284 = t3293 * t14283;
    let t14287 = t1728 * t3096;
    let t14288 = t4642 * t14287;
    let t14291 = t4637 * t2952;
    let t14292 = t3293 * t14291;
    let t14295 = t4852 * t1045;
    let t14296 = t4642 * t14295;
    let t14299 = t3251 * t1762;
    (t14274, t14275, t14278, t14279, t14283, t14284, t14287, t14288, t14291, t14292, t14295, t14296, t14299)
}
