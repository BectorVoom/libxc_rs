//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 181/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk181<F: Float>(t196: F, t618: F, t361: F, t380: F, t383: F, t387: F, t423: F, t430: F, t435: F, t436: F, t500: F, t579: F, t581: F, t589: F) -> (F, F) {
    let t619 = t196 * t618;
    let t622 = t361 + t380 - t383 - t387 + t579 + t423 + t581 - t430 - t435 + F::new(0.93273e-1) * t436 * t589 + F::new(0.31091e-1) * t619 * t500;
    (t619, t622)
}
