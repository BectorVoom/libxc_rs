//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 781/1093 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk781<F: Float>(t1244: F, t21100: F, t1241: F, t5273: F, t5292: F, t1260: F, t6601: F, t140: F, t6652: F, t1222: F, t1234: F, t6594: F, t3172: F, t6630: F, t3600: F, t247: F, t3634: F, t6425: F) -> (F, F, F, F, F, F, F) {
    let t21101 = t1244 * t21100;
    let t21102 = t1241 * t21101;
    let t21107 = t5273 * t5292;
    let t21143 = t6601 * t1260;
    let t21169 = t140 * t6652;
    let t21170 = t1222 * t21169;
    let t21177 = t1234 * t6594;
    let t21188 = t3172 * t6630;
    let t21189 = t3600 * t21188;
    let t21192 = t247 * t3634 * t6425;
    (t21102, t21107, t21143, t21170, t21177, t21189, t21192)
}
