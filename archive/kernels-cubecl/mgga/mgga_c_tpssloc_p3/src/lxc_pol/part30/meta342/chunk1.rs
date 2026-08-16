//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 1377/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1377<F: Float>(t13396: F, t226: F, t4265: F, t814: F, t225: F, t4149: F, t4351: F, t892: F, t1543: F, t2841: F, t4389: F, t699: F) -> (F, F, F, F, F, F) {
    let t13397 = t226 * t13396;
    let t13433 = t814 * t4265;
    let t13463 = t4149 * t225;
    let t13515 = t4351 * t892;
    let t13520 = t1543 * t2841;
    let t13550 = t699 * t4389;
    (t13397, t13433, t13463, t13515, t13520, t13550)
}
