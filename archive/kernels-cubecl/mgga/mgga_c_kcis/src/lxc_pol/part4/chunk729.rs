//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 729/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk729<F: Float>(t4273: F, t572: F, t571: F, t4108: F, t552: F, t577: F, t585: F, t3733: F, t1548: F, t1543: F, t1552: F, t4122: F) -> (F, F, F, F, F, F, F, F, F) {
    let t4274 = t572 * t4273;
    let t4275 = t571 * t4274;
    let t4277 = t4108 * t552;
    let t4278 = t4277 * t577;
    let t4279 = t4278 * t585;
    let t4281 = t3733 * t577;
    let t4282 = t4281 * t1548;
    let t4284 = t1543 * t1552;
    let t4286 = t4122 * t577;
    (t4274, t4275, t4277, t4278, t4279, t4281, t4282, t4284, t4286)
}
