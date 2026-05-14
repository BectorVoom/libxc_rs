//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 868/880 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk868<F: Float>(t1429: F, t14334: F, t193: F, t46379: F, t46382: F, t46384: F, t46387: F, t46390: F, t46396: F, t46398: F, t46404: F, t46408: F, t46420: F, t46422: F, t46426: F, t46432: F, t46435: F, t46447: F, t46450: F, t47994: F, t49827: F, t524: F, t549: F) -> (F,) {
    let t50661 = 0.35750489951850426669e0 * t524 * t14334 * t193 + t46379 + t46382 - 0.76685851907841499352e0 * t46384 + 0.36425779656224712192e1 * t46387 - 0.51762950037793012063e1 * t46390 - t47994 + 0.39722766613167140743e-1 * t1429 * t549 * t49827 + t46396 + 0.76685851907841499352e0 * t46398 + t46404 - t46408 + t46420 + t46422 + t46426 - t46432 + t46435 - t46447 - t46450;
    (t50661,)
}
