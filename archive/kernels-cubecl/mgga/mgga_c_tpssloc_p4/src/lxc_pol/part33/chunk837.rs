//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 837/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk837<F: Float>(t1146: F, t3399: F, t3402: F, t448: F, t445: F, t440: F, t11135: F, t11203: F, t1127: F, t3355: F, t427: F, t3358: F, t435: F) -> (F, F, F, F, F, F, F, F) {
    let t11282 = F::cast_from(1.0_f64) / t3399 / t1146;
    let t11285 = F::cast_from(1.0_f64) / t3402 / t448;
    let t11292 = F::cast_from(1.0_f64) / t3399 / t445;
    let t11310 = t440 * t11282;
    let t11314 = F::cast_from(0.16068111111111111111e1_f64) * t11135;
    let t11317 = F::cast_from(0.46308888888888888888e0_f64) * t11203;
    let t11349 = F::cast_from(1.0_f64) / t3355 / t1127;
    let t11350 = t427 * t11349;
    let t11352 = F::cast_from(1.0_f64) / t3358 / t435;
    (t11282, t11285, t11292, t11310, t11314, t11317, t11350, t11352)
}
