//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1283/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1283<F: Float>(t104519: F, t104525: F, t104527: F, t104529: F, t104532: F, t104541: F, t104549: F, t1349: F, t1647: F, t1651: F, t1969: F, t24064: F, t24066: F, t26567: F, t26780: F, t26783: F, t26791: F, t28: F, t5766: F, t5772: F, t6580: F, t94330: F) -> (F,) {
    let t104550 = -t104519 + t94330 / 27.0 - t1349 * t28 * t26791 * t24064 / 3.0 - 4.0 * t104525 - 2.0 * t104527 + 8.0 * t104529 + t104532 + t5766 * t26780 / 3.0 - t6580 * t24066 / 3.0 + t5772 * t1969 * t26567 * t1647 / 9.0 - 2.0 / 81.0 * t104541 - t5772 * t1969 * t26783 * t1651 / 18.0 + t104549;
    (t104550,)
}
