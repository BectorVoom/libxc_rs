//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 1000/1013 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk1000(t1429: f64, t14334: f64, t193: f64, t46379: f64, t46382: f64, t46384: f64, t46387: f64, t46390: f64, t46396: f64, t46398: f64, t46404: f64, t46408: f64, t46420: f64, t46422: f64, t46426: f64, t46432: f64, t46435: f64, t46447: f64, t46450: f64, t47994: f64, t49827: f64, t524: f64, t549: f64) -> f64 {
    let t50661 = 0.35750489951850426669e0_f64 * t524 * t14334 * t193 + t46379 + t46382 - 0.76685851907841499352e0_f64 * t46384 + 0.36425779656224712192e1_f64 * t46387 - 0.51762950037793012063e1_f64 * t46390 - t47994 + 0.39722766613167140743e-1_f64 * t1429 * t549 * t49827 + t46396 + 0.76685851907841499352e0_f64 * t46398 + t46404 - t46408 + t46420 + t46422 + t46426 - t46432 + t46435 - t46447 - t46450;
    t50661
}
