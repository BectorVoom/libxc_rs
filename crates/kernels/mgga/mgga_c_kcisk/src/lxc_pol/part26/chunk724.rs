//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 724/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk724<F: Float>(t210: F, t9355: F, t9343: F, t9346: F, t9348: F, t9350: F, t9353: F) -> (F, F) {
    let t9356 = t210 * t9355;
    let t9358 = t9343 / 8.0 - t9346 / 8.0 - t9348 / 4.0 - t9350 / 32.0 + t9353 / 32.0 + t9356 / 8.0;
    (t9356, t9358)
}
