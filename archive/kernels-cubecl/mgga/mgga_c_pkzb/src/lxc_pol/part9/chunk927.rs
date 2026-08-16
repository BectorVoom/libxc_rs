//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 927/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk927<F: Float>(t158: F, t7011: F, t7026: F, t7044: F, t7052: F, t133: F, t594: F, t1020: F, t1773: F, t1634: F, t2575: F, t614: F) -> (F, F, F, F, F) {
    let t7055 = (t7011 + t7026 + t7044 + t7052) * t158;
    let t7065 = t594 * t133;
    let t7070 = t1773 * t1020;
    let t7071 = t7070 * t1634;
    let t7074 = t614 * t2575;
    (t7055, t7065, t7070, t7071, t7074)
}
