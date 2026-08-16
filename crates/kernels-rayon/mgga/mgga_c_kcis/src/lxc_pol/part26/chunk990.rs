//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 990/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk990(t238: f64, t7376: f64, t86: f64, t1153: f64, t12431: f64, t17583: f64, t17586: f64, t17613: f64, t17641: f64, t17645: f64, t22063: f64, t22098: f64, t22138: f64, t22547: f64, t22554: f64, t22558: f64, t22562: f64, t22570: f64, t22574: f64, t368: f64, t4202: f64, t5464: f64, t5499: f64, t5958: f64) -> f64 {
    let t22578 = t86 * t238 * t7376;
    let t22581 = 0.123825e-1_f64 * t5958 * t22098 - 0.53062222222222222222e-1_f64 * t1153 * t22547 - 0.371475e-1_f64 * t5958 * t22138 + 0.70749629629629629628e-1_f64 * t17583 - t17586 + 0.53062222222222222222e-1_f64 * t1153 * t22554 - 0.26531111111111111111e-1_f64 * t1153 * t22558 - 0.26531111111111111111e-1_f64 * t1153 * t22562 - 0.1857375e-1_f64 * t17613 * t5464 - 0.1857375e-1_f64 * t4202 * t22063 + 0.58958024691358024691e-2_f64 * t12431 - 0.39796666666666666666e-1_f64 * t86 * t368 * t22570 + 0.24765e-1_f64 * t22574 * t5499 - 0.26531111111111111111e-1_f64 * t22578 + 0.17687407407407407407e-1_f64 * t17641 - t17645;
    t22581
}
