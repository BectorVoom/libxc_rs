//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 888/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk888<F: Float>(t1947: F, t1954: F, t1955: F, t6092: F, t3068: F, t81: F, t1211: F) -> (F, F, F, F, F) {
    let t8074 = t1954 * t1947;
    let t8077 = t6092 * t1955;
    let t8080 = t81 * t3068;
    let t8102 = t1954 * t1211;
    let t8103 = t81 * t1955;
    (t8074, t8077, t8080, t8102, t8103)
}
