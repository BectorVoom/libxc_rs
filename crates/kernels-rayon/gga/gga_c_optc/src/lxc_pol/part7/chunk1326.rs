//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1326/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1326(t3012: f64, t8688: f64, t2994: f64, t8686: f64, t241: f64, t8868: f64, t1104: f64, t8558: f64, t8565: f64, t26237: f64, t26240: f64, t26242: f64, t26245: f64, t26251: f64, t26455: f64, t26457: f64, t26459: f64, t26463: f64) -> (f64, f64, f64, f64) {
    let t26464 = t3012 * t8688;
    let t26467 = 0.3103500882342370105e4_f64 * t8686 * t26464 * t2994;
    let t26468 = t241 * t8868;
    let t26470 = 0.23392893589820816284e1_f64 * t26468 * t1104;
    let t26472 = 24.0_f64 * t8558 * t8565;
    let t26473 = t26237 + t26240 + t26242 + t26245 - t26251 + t26455 - t26457 + t26459 + t26463 + t26467 - t26470 - t26472;
    (t26467, t26470, t26472, t26473)
}
