//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 988/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk988<F: Float>(t30388: F, t30443: F, t30505: F, t30558: F, t1023: F, t1349: F, t1389: F, t149: F, t24118: F, t26575: F, t26789: F, t30169: F, t30172: F, t30281: F, t30285: F, t30290: F, t30297: F, t30302: F, t30304: F, t30306: F, t30324: F, t30358: F, t4650: F, t4720: F, t5772: F, t6580: F, t6618: F, t6622: F, t6723: F) -> (F, F) {
    let t30560 = t30388 + t30443 + t30505 + t30558;
    let t30564 = -t26575 / 9.0 + t5772 * t30169 / 9.0 + 8.0 * t30172 + 2.0 * t30281 + 2.0 / 9.0 * t26789 + 2.0 / 9.0 * t5772 * t30285 + t1349 * t30290 + t6580 * t6618 / 3.0 + t6580 * t6622 / 3.0 + t24118 - t4720 * t1389 - 2.0 * t30297 - 2.0 * t1023 * t6723 - t4650 * t1389 - 2.0 * t30302 - 4.0 * t30304 - 4.0 * t30306 - t149 * t30560 - 2.0 * t30358 + 4.0 * t30324;
    (t30560, t30564)
}
