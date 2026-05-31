//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 586/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk586<F: Float>(t2630: F, t313: F, t1035: F, t829: F, t1045: F, t2635: F, t312: F) -> (F, F, F, F, F) {
    let t3062 = t313 * t2630;
    let t3065 = t1035 * t829;
    let t3066 = t3065 * t1045;
    let t3069 = t313 * t2635;
    let t3072 = t312 * t312;
    let t3073 = F::cast_from(1.0_f64) / t3072;
    (t3062, t3066, t3069, t3072, t3073)
}
