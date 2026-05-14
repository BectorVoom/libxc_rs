//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 543/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk543<F: Float>(t161: F, t3557: F, t1067: F, t806: F, t3223: F, t980: F, t3290: F, t984: F, t204: F, t4091: F, t737: F, t143: F, t3230: F, t3233: F, t178: F, t2971: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t4530 = 5.485926352720394 * t161 * t3557;
    let t4531 = t806 * t1067;
    let t4563 = t980 * t3223;
    let t4566 = 4.937333717448355 * t980 * t3290;
    let t4567 = t984 * t3223;
    let t4570 = 3.7610742193750633 * t984 * t3290;
    let t4571 = t4091 * t204;
    let t4572 = t4571 * t737;
    let t4574 = t143 * t3230;
    let t4576 = t143 * t3233;
    let t4581 = t178 * t2971;
    (t4530, t4531, t4563, t4566, t4567, t4570, t4572, t4574, t4576, t4581)
}
