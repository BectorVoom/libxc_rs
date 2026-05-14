//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1281/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1281<F: Float>(t1347: F, t1900: F, t7149: F, t40424: F, t5773: F, t1349: F, t26792: F, t376: F, t12277: F, t5968: F, t6718: F, t9428: F, t24116: F, t6580: F, t12334: F, t12561: F, t1643: F, t165: F, t1969: F, t23408: F, t26783: F, t26793: F, t26809: F, t26811: F, t28: F, t3051: F, t3052: F, t5765: F, t5766: F, t5772: F, t5778: F, t9049: F, t925: F, t94285: F, t94311: F, t94327: F) -> (F, F, F, F) {
    let t104477 = t1347 * t7149 * t1900;
    let t104478 = t40424 * t5773;
    let t104484 = 2.0 / 9.0 * t1349 * t376 * t26792;
    let t104496 = t12277 * t5968;
    let t104498 = t9428 * t6718;
    let t104512 = t6580 * t24116;
    let t104515 = -2.0 / 3.0 * t104477 * t104478 * t12334 + t104484 - t94311 / 18.0 - 2.0 / 3.0 * t5766 * t26793 - t5772 * t9049 * t26783 * t1643 / 27.0 - t5772 * t1969 * t94285 * t925 / 18.0 - 4.0 * t104496 - 2.0 * t104498 - 2.0 / 9.0 * t5765 * t3051 * t26811 - 2.0 / 9.0 * t26809 * t1969 * t23408 * t3052 - t1349 * t28 * t5778 * t165 * t12561 / 3.0 + 2.0 / 27.0 * t104512 - t94327 / 27.0;
    (t104477, t104496, t104498, t104515)
}
