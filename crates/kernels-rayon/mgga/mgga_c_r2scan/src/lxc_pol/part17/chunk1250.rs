//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1250/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1250(t11497: f64, t12056: f64, t3262: f64, t11199: f64, t12391: f64, t11336: f64, t37327: f64, t43767: f64, t31498: f64, t3275: f64, t3465: f64, t12574: f64, t39040: f64) -> (f64, f64, f64, f64, f64) {
    let t44541 = 3.0_f64 / 2.0_f64 * t3262 * t12056 * t11497;
    let t44544 = 3.0_f64 / 2.0_f64 * t3262 * t11199 * t12391;
    let t44548 = 15.0_f64 / 8.0_f64 * t37327 * t11336 * t43767;
    let t44551 = t3275 * t3465 * t31498 / 4.0_f64;
    let t44554 = 45.0_f64 / 64.0_f64 * t3275 * t39040 * t12574;
    (t44541, t44544, t44548, t44551, t44554)
}
