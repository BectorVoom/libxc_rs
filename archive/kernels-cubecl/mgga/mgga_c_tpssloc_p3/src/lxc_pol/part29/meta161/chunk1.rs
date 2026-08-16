//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 854/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk854<F: Float>(t3036: F, t483: F, t3503: F, t3500: F) -> (F, F, F) {
    let t3504 = t483 * t3036;
    let t3505 = t3503 * t3504;
    let t3506 = t3500 * t3505;
    (t3504, t3505, t3506)
}
