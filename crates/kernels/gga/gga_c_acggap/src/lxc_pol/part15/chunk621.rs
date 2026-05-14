//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 621/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk621<F: Float>(t6421: F, t6441: F, t6453: F, t6574: F, t1717: F, t3952: F, t1941: F, t814: F, t157: F, t513: F, t524: F, t506: F, t1838: F, t944: F, t104: F, t566: F, t95: F) -> (F, F, F, F, F, F, F) {
    let t6576 = t6421 + t6441 + t6453 + t6574;
    let t6596 = t1717 * t3952;
    let t6614 = t1941 * t814;
    let t6841 = t513 * t524 * t157;
    let t6847 = t506 * t524 * t157;
    let t7158 = t944 * t1838;
    let t7297 = t566 * t95 * t104;
    (t6576, t6596, t6614, t6841, t6847, t7158, t7297)
}
