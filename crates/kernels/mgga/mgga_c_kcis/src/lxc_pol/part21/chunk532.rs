//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 532/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk532<F: Float>(t1154: F, t3405: F, t829: F, t1071: F, t339: F, t2630: F, t1155: F, t2635: F, t304: F, t3166: F, t1110: F, t1115: F, t1143: F, t1153: F, t3289: F, t3295: F, t3299: F, t3304: F, t3308: F, t3372: F, t3381: F, t3392: F, t3394: F, t3397: F, t3402: F, t348: F, t365: F, t368: F, t86: F) -> (F, F, F, F, F) {
    let t3407 = t1154 * t3405 * t829;
    let t3410 = t339 * t1071;
    let t3412 = t1154 * t3410 * t2630;
    let t3416 = t1154 * t1155 * t2635;
    let t3419 = t304 * t3166;
    let t3423 = F::new(0.619125e-2) * t3372 * t348 + F::new(0.1857375e-1) * t1143 * t1110 - F::new(0.123825e-1) * t1143 * t1115 + F::new(0.46434375e-2) * t365 * t3289 - F::new(0.1857375e-1) * t3381 * t3295 + F::new(0.9286875e-2) * t365 * t3299 + F::new(0.123825e-1) * t365 * t3304 - F::new(0.619125e-2) * t365 * t3308 + t3392 - F::cast_from(0.35374814814814814814e-1_f64) * t3394 - F::cast_from(0.53062222222222222222e-1_f64) * t3397 - F::cast_from(0.44218518518518518518e-1_f64) * t1153 * t3402 - F::cast_from(0.53062222222222222222e-1_f64) * t1153 * t3407 + F::cast_from(0.53062222222222222222e-1_f64) * t1153 * t3412 - F::cast_from(0.26531111111111111111e-1_f64) * t1153 * t3416 - F::cast_from(0.39796666666666666666e-1_f64) * t86 * t368 * t3419;
    (t3407, t3412, t3416, t3419, t3423)
}
