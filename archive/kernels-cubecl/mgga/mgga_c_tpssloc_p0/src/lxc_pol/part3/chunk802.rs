//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 3 (v3rho3_1) CSE chunk 802/1255 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part3_v3rho3_1_chunk802<F: Float>(t1637: F, t3216: F, t1068: F, t1070: F, t193: F, t336: F, t4353: F, t4356: F, t4358: F, t4361: F, t4398: F, t4402: F, t4480: F, t4482: F, t4485: F, t4487: F, t4491: F, t4495: F, t4500: F, t4696: F, t4700: F) -> (F, F) {
    let t4701 = t1637 * t3216;
    let t4704 = t1070 * t193 * t336 * t4696 - t1068 * t4700 * t4701 - t4353 + t4356 + t4358 - t4361 + t4398 + t4402 + t4480 + t4482 - t4485 - t4487 + t4491 - t4495 - t4500;
    (t4701, t4704)
}
