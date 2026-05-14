//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 848/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk848<F: Float>(t5728: F, t6517: F, t6461: F, t758: F, t2362: F, t5717: F) -> (F, F, F, F) {
    let t6518 = t5728 * t6517;
    let t6519 = t6461 * t6518;
    let t6520 = t758 * t6519;
    let t6523 = t5717 * t2362;
    (t6518, t6519, t6520, t6523)
}
