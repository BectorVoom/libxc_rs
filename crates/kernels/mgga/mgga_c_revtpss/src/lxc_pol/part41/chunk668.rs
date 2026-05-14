//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 668/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk668<F: Float>(t108: F, t2: F, t580: F, t105: F, t1505: F, t1507: F, t4270: F, t4274: F, t4280: F, t656: F, t662: F, t97: F) -> (F, F) {
    let t4283 = t108 * t2;
    let t4284 = t4283 * t580;
    let t4287 = -25.0 / 9.0 * t656 * t1505 + 10.0 / 9.0 * t97 * t4270 + 5.0 / 3.0 * t97 * t4274 - 25.0 / 9.0 * t1507 * t662 + 10.0 / 9.0 * t105 * t4280 - 5.0 / 3.0 * t105 * t4284;
    (t4284, t4287)
}
