//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1370/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1370<F: Float>(t238: F, t7376: F, t86: F, t1153: F, t12431: F, t17583: F, t17586: F, t17613: F, t17641: F, t17645: F, t22063: F, t22098: F, t22138: F, t22547: F, t22554: F, t22558: F, t22562: F, t22570: F, t22574: F, t368: F, t4202: F, t5464: F, t5499: F, t5958: F) -> F {
    let t22578 = t86 * t238 * t7376;
    let t22581 = F::new(0.123825e-1) * t5958 * t22098 - F::new(0.53062222222222222222e-1) * t1153 * t22547 - F::new(0.371475e-1) * t5958 * t22138 + F::new(0.70749629629629629628e-1) * t17583 - t17586 + F::new(0.53062222222222222222e-1) * t1153 * t22554 - F::new(0.26531111111111111111e-1) * t1153 * t22558 - F::new(0.26531111111111111111e-1) * t1153 * t22562 - F::new(0.1857375e-1) * t17613 * t5464 - F::new(0.1857375e-1) * t4202 * t22063 + F::new(0.58958024691358024691e-2) * t12431 - F::new(0.39796666666666666666e-1) * t86 * t368 * t22570 + F::new(0.24765e-1) * t22574 * t5499 - F::new(0.26531111111111111111e-1) * t22578 + F::new(0.17687407407407407407e-1) * t17641 - t17645;
    t22581
}
