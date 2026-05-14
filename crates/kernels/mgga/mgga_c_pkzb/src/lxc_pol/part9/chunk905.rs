//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 905/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk905<F: Float>(t722: F, t7474: F, t2826: F, t713: F, t1070: F, t1854: F, t1857: F, t1088: F, t1915: F, t2743: F, t663: F, t685: F, t1894: F, t2746: F, t1100: F, t1954: F) -> (F, F, F, F, F, F, F, F, F) {
    let t7475 = t7474 * t722;
    let t7478 = t2826 * t713;
    let t7483 = t1070 * t1854;
    let t7485 = 2.0 * t7483 * t1857;
    let t7486 = t1088 * t1915;
    let t7489 = t2743 * t663;
    let t7491 = 2.0 * t7489 * t685;
    let t7493 = 1.0 * t2746 * t1894;
    let t7494 = t1100 * t1954;
    (t7475, t7478, t7483, t7485, t7486, t7489, t7491, t7493, t7494)
}
