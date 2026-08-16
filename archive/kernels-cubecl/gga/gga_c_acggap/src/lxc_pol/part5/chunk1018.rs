//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1018/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1018<F: Float>(t13287: F, t13293: F, t1449: F, t4210: F, t13364: F, t5122: F, t8401: F, t13299: F, t5127: F, t1095: F, t3101: F, t384: F, t398: F, t513: F) -> (F, F, F, F) {
    let t17258 = t13293 * t13287 * t1449 * t4210;
    let t17262 = t13293 * t13364 * t8401 * t5122;
    let t17266 = t13293 * t13299 * t8401 * t5127;
    let t17281 = t384 * t398 * t1095 * t513 * t3101;
    (t17258, t17262, t17266, t17281)
}
