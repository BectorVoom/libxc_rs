//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 710/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk710<F: Float>(t4546: F, t4547: F, t3210: F, t3200: F, t2840: F, t341: F, t1017: F, t86: F) -> (F, F, F, F, F) {
    let t4548 = t4546 * t4547;
    let t4549 = t3210 * t4548;
    let t4550 = t3200 * t4549;
    let t4552 = t2840 * t341;
    let t4554 = t86 * t1017 * t4552;
    (t4548, t4549, t4550, t4552, t4554)
}
