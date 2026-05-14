//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1172/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1172<F: Float>(t18796: F, t3038: F, t6317: F, t8189: F, t8192: F, t18790: F, t8195: F, t18609: F, t3074: F, t6137: F, t8199: F, t8202: F, t18612: F, t8206: F, t2197: F, t8004: F, t851: F) -> (F, F, F, F, F, F, F, F, F) {
    let t22530 = 6.0 * t18796 * t3038;
    let t22532 = 12.0 * t6317 * t8189;
    let t22534 = 6.0 * t6317 * t8192;
    let t22536 = 0.28947563097646563121e3 * t18790 * t8195;
    let t22538 = 0.48245938496077605201e2 * t18609 * t3074;
    let t22540 = 0.96491876992155210402e2 * t6137 * t8199;
    let t22542 = 0.48245938496077605201e2 * t6137 * t8202;
    let t22544 = 0.1551780387578202009e4 * t18612 * t8206;
    let t22547 = 6.0 * t2197 * t8004 * t851;
    (t22530, t22532, t22534, t22536, t22538, t22540, t22542, t22544, t22547)
}
