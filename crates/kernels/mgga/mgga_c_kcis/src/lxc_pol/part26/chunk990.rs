//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 990/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk990<F: Float>(t238: F, t7376: F, t86: F, t1153: F, t12431: F, t17583: F, t17586: F, t17613: F, t17641: F, t17645: F, t22063: F, t22098: F, t22138: F, t22547: F, t22554: F, t22558: F, t22562: F, t22570: F, t22574: F, t368: F, t4202: F, t5464: F, t5499: F, t5958: F) -> F {
    let t22578 = t86 * t238 * t7376;
    let t22581 = F::cast_from(0.123825e-1_f64) * t5958 * t22098 - F::cast_from(0.53062222222222222222e-1_f64) * t1153 * t22547 - F::cast_from(0.371475e-1_f64) * t5958 * t22138 + F::cast_from(0.70749629629629629628e-1_f64) * t17583 - t17586 + F::cast_from(0.53062222222222222222e-1_f64) * t1153 * t22554 - F::cast_from(0.26531111111111111111e-1_f64) * t1153 * t22558 - F::cast_from(0.26531111111111111111e-1_f64) * t1153 * t22562 - F::cast_from(0.1857375e-1_f64) * t17613 * t5464 - F::cast_from(0.1857375e-1_f64) * t4202 * t22063 + F::cast_from(0.58958024691358024691e-2_f64) * t12431 - F::cast_from(0.39796666666666666666e-1_f64) * t86 * t368 * t22570 + F::cast_from(0.24765e-1_f64) * t22574 * t5499 - F::cast_from(0.26531111111111111111e-1_f64) * t22578 + F::cast_from(0.17687407407407407407e-1_f64) * t17641 - t17645;
    t22581
}
