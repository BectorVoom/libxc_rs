//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 712/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk712<F: Float>(t1495: F, t4129: F, t1468: F, t1464: F, t584: F) -> (F, F, F, F) {
    let t4130 = t1495 * t4129;
    let t4131 = t1468 * t4130;
    let t4132 = t1464 * t4131;
    let t4134 = F::new(1.0) / t584;
    (t4130, t4131, t4132, t4134)
}
