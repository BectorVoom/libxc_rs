//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 840/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk840<F: Float>(t41: F, t5821: F, t698: F, t445: F, t1836: F, t3114: F, t1843: F, t3119: F, t1857: F, t3123: F, t1060: F, t1846: F, t1849: F, t213: F, t4597: F, t967: F) -> (F, F, F, F, F, F, F, F) {
    let t11529 = t5821 * t41;
    let t11530 = t11529 * t698;
    let t11532 = 0.72818958333333333333e-4 * t445 * t11530;
    let t11533 = t3114 * t1836;
    let t11535 = t3119 * t1843;
    let t11562 = t3123 * t1857;
    let t11605 = t1846 * t1060;
    let t11612 = t213 * t1849;
    let t11613 = t11612 * t1060;
    let t11625 = t967 * t4597;
    (t11532, t11533, t11535, t11562, t11605, t11612, t11613, t11625)
}
