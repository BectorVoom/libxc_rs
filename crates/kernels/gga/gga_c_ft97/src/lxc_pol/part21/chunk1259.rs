//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1259/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1259<F: Float>(t12664: F, t26523: F, t26526: F, t104623: F, t1053: F, t1023: F, t119477: F, t119479: F, t119482: F, t119484: F, t119486: F, t119488: F, t119492: F, t119496: F, t1389: F, t16664: F, t1969: F, t27406: F, t30034: F, t379: F, t5766: F, t5772: F) -> (F, F, F, F) {
    let t119501 = t12664 * t26523;
    let t119503 = t12664 * t26526;
    let t119505 = t104623 * t1053;
    let t119509 = 4.0 * t119477 + 4.0 * t119479 + 4.0 * t119482 - 12.0 * t119484 + 8.0 * t119486 - 4.0 * t119488 - t16664 * t1389 - 2.0 * t119492 - 2.0 * t1023 * t27406 - t5772 * t1969 * t119496 * t379 / 18.0 + 8.0 * t119501 + 8.0 * t119503 - 4.0 * t119505 + t5766 * t30034 / 3.0;
    (t119501, t119503, t119505, t119509)
}
