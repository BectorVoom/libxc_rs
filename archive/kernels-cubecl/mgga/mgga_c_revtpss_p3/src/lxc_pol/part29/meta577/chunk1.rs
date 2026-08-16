//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1927/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1927<F: Float>(t14751: F, t7045: F, t14757: F, t25234: F, t14738: F, t7038: F, t14732: F, t25245: F, t14668: F, t27261: F, t14933: F, t2482: F, t25260: F, t814: F) -> (F, F, F, F, F, F) {
    let t99071 = t7045 * t14751;
    let t99073 = t25234 * t14757;
    let t99075 = t7038 * t14738;
    let t99077 = t25245 * t14732;
    let t99081 = t27261 * t14668;
    let t99085 = t2482 * t25260 * t814 * t14933;
    (t99071, t99073, t99075, t99077, t99081, t99085)
}
