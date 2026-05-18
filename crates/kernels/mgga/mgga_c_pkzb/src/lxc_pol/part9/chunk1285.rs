//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1285/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1285<F: Float>(t6327: F, t8009: F, t18796: F, t3038: F, t6317: F, t8189: F, t8192: F, t18790: F, t8195: F, t18609: F, t3074: F, t6137: F, t8199: F) -> (F, F, F, F, F, F, F) {
    let t22528 = F::new(6.0) * t8009 * t6327;
    let t22530 = F::new(6.0) * t18796 * t3038;
    let t22532 = F::new(12.0) * t6317 * t8189;
    let t22534 = F::new(6.0) * t6317 * t8192;
    let t22536 = F::new(0.28947563097646563121e3) * t18790 * t8195;
    let t22538 = F::new(0.48245938496077605201e2) * t18609 * t3074;
    let t22540 = F::new(0.96491876992155210402e2) * t6137 * t8199;
    (t22528, t22530, t22532, t22534, t22536, t22538, t22540)
}
