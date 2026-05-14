//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 994/1093 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk994<F: Float>(t23059: F, t4147: F, t7036: F, t820: F, t844: F, t2482: F, t814: F, t228: F, t25273: F, t25282: F, t9802: F, t243: F, t7021: F, t1941: F, t853: F, t64: F, t9731: F) -> (F, F, F, F, F, F, F, F) {
    let t86825 = t23059 * t4147;
    let t92951 = t820 * t7036 * t844;
    let t92955 = t2482 * t7036 * t814;
    let t92968 = t25273 * t228;
    let t92975 = t9802 * t25282;
    let t92978 = t7021 * t243;
    let t92981 = t1941 * t853;
    let t92986 = t64 * t9731;
    (t86825, t92951, t92955, t92968, t92975, t92978, t92981, t92986)
}
