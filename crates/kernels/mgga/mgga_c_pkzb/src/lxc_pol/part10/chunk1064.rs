//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1064/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1064<F: Float>(t713: F, t722: F, t9203: F, t730: F, t2860: F, t2866: F, t1987: F, t3622: F, t2751: F, t7483: F, t2787: F, t7411: F, t3525: F, t683: F, t1899: F, t1084: F, t2782: F) -> (F, F, F, F, F, F, F, F, F) {
    let t9205 = t713 * t9203 * t722;
    let t9207 = 0.5848223622634646207e0 * t730 * t9205;
    let t9209 = 0.23392894490538584828e1 * t2860 * t2866;
    let t9211 = 0.5848223622634646207e0 * t1987 * t3622;
    let t9213 = 4.0 * t7483 * t2751;
    let t9215 = 0.32163958997385070134e2 * t7411 * t2787;
    let t9216 = t3525 * t683;
    let t9218 = 6.0 * t1899 * t9216;
    let t9219 = t1084 * t2782;
    (t9205, t9207, t9209, t9211, t9213, t9215, t9216, t9218, t9219)
}
