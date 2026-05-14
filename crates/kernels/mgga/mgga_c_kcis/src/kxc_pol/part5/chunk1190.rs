//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1190/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1190<F: Float>(t21078: F, t5425: F, t531: F, t6957: F, t11332: F, t833: F, t544: F, t6964: F, t1319: F, t5457: F, t518: F, t1419: F, t3786: F, t16029: F, t1961: F, t3255: F, t7226: F) -> (F, F, F, F, F, F, F, F, F) {
    let t22044 = t5425 * t21078;
    let t22047 = t6957 * t531;
    let t22049 = t11332 * t22047 * t833;
    let t22053 = t544 * t6964;
    let t22054 = t22053 * t1319;
    let t22055 = t5457 * t22054;
    let t22058 = t518 * t6964;
    let t22059 = t22058 * t1419;
    let t22060 = t3786 * t22059;
    let t22063 = t16029 * t1961;
    let t22064 = t3786 * t22063;
    let t22067 = t3255 * t7226;
    (t22044, t22049, t22054, t22055, t22059, t22060, t22063, t22064, t22067)
}
