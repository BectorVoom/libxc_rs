//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3177/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3177<F: Float>(t3476: F, t5117: F, t3451: F, t3383: F, t5060: F, t12247: F, t1719: F, t1756: F, t3521: F, t1130: F, t16807: F, t3432: F) -> (F, F, F, F, F, F, F) {
    let t58317 = t5117 * t3476;
    let t58336 = t5117 * t3451;
    let t58339 = t5060 * t3383;
    let t58342 = t1719 * t12247;
    let t58345 = t3521 * t1756;
    let t58460 = t16807 * t1130;
    let t58466 = t5060 * t3432;
    (t58317, t58336, t58339, t58342, t58345, t58460, t58466)
}
