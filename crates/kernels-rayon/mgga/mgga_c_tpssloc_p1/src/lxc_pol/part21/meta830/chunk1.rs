//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2927/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2927(t17938: f64, t2940: f64, t13663: f64, t4483: f64, t14259: f64, t41825: f64, t5774: f64, t959: f64, t17566: f64, t3213: f64, t43637: f64, t4700: f64, t5950: f64, t60359: f64, t60371: f64, t60374: f64, t60377: f64, t60381: f64, t60384: f64, t60387: f64, t60391: f64, t60394: f64) -> (f64, f64, f64, f64, f64) {
    let t60930 = 0.23392894490538584828e1_f64 * t2940 * t17938;
    let t60932 = 0.46785788981077169656e1_f64 * t4483 * t13663;
    let t60936 = 0.12304822629859687989e5_f64 * t959 * t41825 * t5774 * t14259;
    let t60938 = 0.20508037716432813316e4_f64 * t2940 * t17566;
    let t60939 = -6.0_f64 * t3213 * t43637 * t4700 * t5950 - t60359 - t60371 - t60374 + t60377 + t60381 + t60384 + t60387 + t60391 + t60394 + t60930 + t60932 + t60936 - t60938;
    (t60930, t60932, t60936, t60938, t60939)
}
