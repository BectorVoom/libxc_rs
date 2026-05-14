//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 580/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk580<F: Float>(t1130: F, t330: F, t1133: F, t829: F, t3210: F, t3200: F, t388: F) -> (F, F, F, F, F) {
    let t3211 = t1130 * t330;
    let t3212 = t829 * t1133;
    let t3213 = t3211 * t3212;
    let t3214 = t3210 * t3213;
    let t3215 = t3200 * t3214;
    let t3217 = 1.0 / t388;
    (t3211, t3212, t3214, t3215, t3217)
}
