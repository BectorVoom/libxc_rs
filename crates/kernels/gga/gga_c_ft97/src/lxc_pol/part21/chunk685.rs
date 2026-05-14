//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 685/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk685<F: Float>(t1022: F, t16658: F, t3413: F, t4649: F, t1952: F, t4719: F, t3450: F, t925: F) -> (F, F, F, F) {
    let t16659 = t16658 * t1022;
    let t16661 = t4649 * t3413;
    let t16664 = t1952 * t4719;
    let t16666 = t925 * t3450;
    (t16659, t16661, t16664, t16666)
}
