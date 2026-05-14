//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 692/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk692<F: Float>(t3733: F, t577: F, t1548: F, t1543: F, t1552: F, t4122: F, t4124: F, t584: F, t583: F, t582: F) -> (F, F, F, F, F, F, F, F) {
    let t4281 = t3733 * t577;
    let t4282 = t4281 * t1548;
    let t4284 = t1543 * t1552;
    let t4286 = t4122 * t577;
    let t4287 = t584 * t4124;
    let t4288 = t583 * t4287;
    let t4289 = t4286 * t4288;
    let t4291 = t577 * t582;
    (t4281, t4282, t4284, t4286, t4287, t4288, t4289, t4291)
}
