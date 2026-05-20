//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1947/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1947<F: Float>(t11064: F, t1468: F, t27384: F, t605: F, t6079: F, t5824: F, t890: F, t6075: F, t27383: F, t18392: F, t30: F, t1583: F, t4343: F) -> (F, F, F, F, F, F, F, F) {
    let t106589 = t11064 * t1468;
    let t106590 = t106589 * t27384;
    let t106593 = t605 * t6079;
    let t106602 = t5824 * t890;
    let t106606 = t605 * t6075;
    let t106610 = t6075 * t890;
    let t106611 = t27383 * t106610;
    let t106618 = t30 * t18392;
    let t106625 = t4343 * t1583;
    (t106590, t106593, t106602, t106606, t106610, t106611, t106618, t106625)
}
