//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1119/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1119<F: Float>(t14117: F, t14118: F, t3255: F, t4603: F, t4608: F, t1071: F, t1114: F, t13791: F, t347: F, t4625: F, t934: F, t4600: F) -> (F, F, F, F, F, F) {
    let t14119 = t14117 * t14118;
    let t14125 = F::new(0.98556445e-3) * t3255 * t4603;
    let t14127 = F::new(0.19711289e-2) * t3255 * t4608;
    let t14128 = t1114 * t1071;
    let t14129 = t14128 * t13791;
    let t14132 = t347 * t4625;
    let t14133 = t14132 * t934;
    let t14134 = t4600 * t14133;
    (t14119, t14125, t14127, t14129, t14133, t14134)
}
