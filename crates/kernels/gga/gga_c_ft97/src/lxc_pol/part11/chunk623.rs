//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 623/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk623<F: Float>(t2097: F, t8315: F, t3499: F, t7807: F, t583: F, t8282: F, t462: F, t9178: F, t9179: F, t9181: F, t9183: F, t9186: F, t9188: F, t9190: F, t9193: F, t92: F) -> (F, F, F) {
    let t9196 = t2097 * t8315;
    let t9199 = t3499 * t7807;
    let t9202 = t8282 * t583;
    let t9204 = -t9178 - 4.0 / 3.0 * t9179 + t462 * t9181 + t462 * t9183 - t92 * t9186 - 2.0 / 3.0 * t9188 - 2.0 / 3.0 * t9190 + 2.0 / 3.0 * t462 * t9193 + 4.0 / 3.0 * t462 * t9196 - 2.0 / 3.0 * t462 * t9199 - 4.0 / 9.0 * t9202;
    (t9196, t9199, t9204)
}
