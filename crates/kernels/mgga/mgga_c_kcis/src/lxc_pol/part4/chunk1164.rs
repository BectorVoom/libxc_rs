//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1164/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1164<F: Float>(t3474: F, t5043: F, t1804: F, t3361: F, t375: F, t3477: F, t5068: F, t14058: F, t355: F, t381: F, t389: F, t1180: F, t5165: F) -> (F, F, F, F, F) {
    let t14751 = t3474 * t5043;
    let t14753 = t3361 * t1804;
    let t14754 = t375 * t14753;
    let t14756 = t3477 * t5068;
    let t14758 = t14058 * t355;
    let t14759 = t14758 * t381;
    let t14760 = t14759 * t389;
    let t14762 = t5165 * t1180;
    (t14751, t14754, t14756, t14760, t14762)
}
