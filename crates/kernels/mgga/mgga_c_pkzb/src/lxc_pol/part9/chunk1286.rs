//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1286/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1286<F: Float>(t6137: F, t8202: F, t18612: F, t8206: F, t2197: F, t8004: F, t851: F, t2234: F, t3070: F, t2198: F, t6142: F, t8198: F) -> (F, F, F, F, F) {
    let t22542 = F::new(0.48245938496077605201e2) * t6137 * t8202;
    let t22544 = F::new(0.1551780387578202009e4) * t18612 * t8206;
    let t22547 = F::new(6.0) * t2197 * t8004 * t851;
    let t22550 = F::new(6.0) * t2197 * t3070 * t2234;
    let t22553 = F::new(0.28947563097646563121e3) * t6142 * t8198 * t2198;
    (t22542, t22544, t22547, t22550, t22553)
}
