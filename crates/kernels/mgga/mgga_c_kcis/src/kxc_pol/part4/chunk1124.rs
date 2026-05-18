//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1124/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1124<F: Float>(t1045: F, t14196: F, t4647: F, t3255: F, t4639: F, t4644: F, t3074: F, t4848: F, t4642: F, t313: F, t4670: F, t934: F) -> (F, F, F, F, F, F) {
    let t14198 = t14196 * t4647 * t1045;
    let t14202 = F::new(0.19711289e-2) * t3255 * t4639;
    let t14204 = F::new(0.26281718666666666666e-2) * t3255 * t4644;
    let t14205 = t4848 * t3074;
    let t14206 = t4642 * t14205;
    let t14209 = t313 * t4670;
    let t14210 = t14209 * t934;
    (t14198, t14202, t14204, t14205, t14206, t14210)
}
