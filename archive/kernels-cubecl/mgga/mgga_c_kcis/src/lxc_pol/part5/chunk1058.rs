//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1058/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1058<F: Float>(t3754: F, t822: F, t5851: F, t733: F, t5854: F, t5845: F, t743: F, t5848: F, t1971: F, t2471: F, t1976: F, t2475: F) -> (F, F, F, F, F, F, F) {
    let t17146 = t822 * t3754;
    let t17150 = F::cast_from(0.18736e-1_f64) * t733 * t5851;
    let t17151 = t733 * t5854;
    let t17174 = F::cast_from(0.4705225e-4_f64) * t743 * t5845;
    let t17175 = t743 * t5848;
    let t17197 = t2471 * t1971;
    let t17199 = t2475 * t1976;
    (t17146, t17150, t17151, t17174, t17175, t17197, t17199)
}
