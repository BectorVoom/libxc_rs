//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 805/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk805<F: Float>(t201: F, t6082: F, t199: F, t204: F, t334: F, t3981: F) -> (F, F) {
    let t6083 = t201 * t6082;
    let t6084 = t199 * t6083;
    let t6085 = 0.2390625e-1 * t6084;
    let t6087 = t204 * t3981 * t334;
    (t6085, t6087)
}
