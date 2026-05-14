//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1298/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1298<F: Float>(t29951: F, t6086: F, t6093: F, t1616: F, t2201: F, t3216: F, t785: F, t30856: F, t22709: F, t5108: F, t8760: F, t6518: F, t9289: F, t1584: F, t9226: F, t1632: F, t551: F, t574: F, t8692: F) -> (F, F, F, F, F, F, F) {
    let t30921 = t6093 * t6086 * t29951;
    let t30964 = t2201 * t785 * t1616 * t3216;
    let t30969 = t6093 * t6086 * t30856;
    let t30988 = t5108 * t22709 * t8760;
    let t31018 = t6518 * t9289;
    let t31020 = t1584 * t9226;
    let t31024 = t574 * t551 * t1632 * t8692;
    (t30921, t30964, t30969, t30988, t31018, t31020, t31024)
}
