//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 629/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk629(t1154: f64, t3405: f64, t829: f64, t1071: f64, t339: f64, t2630: f64, t1155: f64, t2635: f64, t304: f64, t3166: f64, t1110: f64, t1115: f64, t1143: f64, t1153: f64, t3289: f64, t3295: f64, t3299: f64, t3304: f64, t3308: f64, t3372: f64, t3381: f64, t3392: f64, t3394: f64, t3397: f64, t3402: f64, t348: f64, t365: f64, t368: f64, t86: f64) -> (f64, f64, f64, f64, f64) {
    let t3407 = t1154 * t3405 * t829;
    let t3410 = t339 * t1071;
    let t3412 = t1154 * t3410 * t2630;
    let t3416 = t1154 * t1155 * t2635;
    let t3419 = t304 * t3166;
    let t3423 = 0.619125e-2_f64 * t3372 * t348 + 0.1857375e-1_f64 * t1143 * t1110 - 0.123825e-1_f64 * t1143 * t1115 + 0.46434375e-2_f64 * t365 * t3289 - 0.1857375e-1_f64 * t3381 * t3295 + 0.9286875e-2_f64 * t365 * t3299 + 0.123825e-1_f64 * t365 * t3304 - 0.619125e-2_f64 * t365 * t3308 + t3392 - 0.35374814814814814814e-1_f64 * t3394 - 0.53062222222222222222e-1_f64 * t3397 - 0.44218518518518518518e-1_f64 * t1153 * t3402 - 0.53062222222222222222e-1_f64 * t1153 * t3407 + 0.53062222222222222222e-1_f64 * t1153 * t3412 - 0.26531111111111111111e-1_f64 * t1153 * t3416 - 0.39796666666666666666e-1_f64 * t86 * t368 * t3419;
    (t3407, t3412, t3416, t3419, t3423)
}
