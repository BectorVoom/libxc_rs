//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1016/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1016<F: Float>(t3255: F, t4603: F, t4608: F, t1071: F, t1114: F, t4634: F, t4597: F, t1035: F, t3293: F, t1727: F, t934: F, t313: F, t4600: F) -> (F, F, F, F, F, F, F, F) {
    let t14125 = F::new(0.98556445e-3) * t3255 * t4603;
    let t14127 = F::new(0.19711289e-2) * t3255 * t4608;
    let t14128 = t1114 * t1071;
    let t14137 = t3255 * t4634;
    let t14168 = F::new(0.13140859333333333333e-2) * t3255 * t4597;
    let t14170 = t3293 * t1035;
    let t14171 = t1727 * t934;
    let t14196 = t4600 * t313;
    (t14125, t14127, t14128, t14137, t14168, t14170, t14171, t14196)
}
