//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1230/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1230<F: Float>(t26788: F, t6580: F, t30156: F, t5769: F, t104161: F, t104204: F, t104205: F, t104331: F, t1058: F, t1349: F, t16150: F, t165: F, t16919: F, t1969: F, t23925: F, t26551: F, t26567: F, t26771: F, t26791: F, t26800: F, t27426: F, t28: F, t30122: F, t30124: F, t3408: F, t3450: F, t5766: F, t5772: F, t5778: F, t925: F, t9432: F) -> (F,) {
    let t118643 = t6580 * t26788;
    let t118667 = t30156 * t5769;
    let t118673 = 2.0 / 9.0 * t5772 * t27426 * t104331 * t16150 + 2.0 / 9.0 * t5772 * t104161 * t26800 - 2.0 / 3.0 * t1349 * t28 * t26791 * t26551 + 2.0 / 9.0 * t118643 - 2.0 / 3.0 * t1349 * t28 * t5778 * t1058 * t3408 - t5766 * t30124 / 3.0 - t1349 * t28 * t23925 * t30122 / 3.0 - t5772 * t1969 * t104205 * t925 / 9.0 - t1349 * t28 * t5778 * t165 * t16919 / 3.0 + t6580 * t26771 / 3.0 - t118667 / 18.0 + 2.0 * t5772 * t9432 * t26567 * t3450 + t104204;
    (t118673,)
}
