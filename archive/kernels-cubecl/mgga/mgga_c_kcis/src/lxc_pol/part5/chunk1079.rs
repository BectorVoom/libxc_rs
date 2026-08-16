//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1079/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1079<F: Float>(t1020: F, t18473: F, t2811: F, t6544: F, t1008: F, t6301: F, t9985: F, t4781: F, t4977: F, t2861: F, t6563: F, t4999: F, t5013: F) -> (F, F, F, F, F, F) {
    let t18474 = t1020 * t18473;
    let t18476 = t6544 * t2811;
    let t18477 = t18476 * t1008;
    let t18482 = t6301 * t9985;
    let t18483 = t18482 * t1008;
    let t18486 = t4781 * t4977;
    let t18495 = t2861 * t6563;
    let t18497 = t4999 * t5013;
    (t18474, t18477, t18483, t18486, t18495, t18497)
}
