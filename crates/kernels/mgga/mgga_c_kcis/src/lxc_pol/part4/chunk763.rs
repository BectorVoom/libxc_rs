//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 763/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk763<F: Float>(t1670: F, t347: F, t934: F, t4600: F, t313: F, t1045: F, t3293: F, t1663: F, t653: F) -> (F, F, F, F, F, F, F) {
    let t4601 = t347 * t1670;
    let t4602 = t4601 * t934;
    let t4603 = t4600 * t4602;
    let t4606 = t313 * t1670;
    let t4607 = t4606 * t1045;
    let t4608 = t3293 * t4607;
    let t4612 = t653 * t1663;
    (t4601, t4602, t4603, t4606, t4607, t4608, t4612)
}
