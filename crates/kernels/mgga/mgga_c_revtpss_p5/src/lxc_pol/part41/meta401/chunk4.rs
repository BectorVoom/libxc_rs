//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 1378/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1378<F: Float>(t1234: F, t6594: F, t1214: F, t5825: F, t5296: F, t1042: F, t3172: F, t6630: F, t3600: F, t247: F, t3634: F, t6425: F) -> (F, F, F, F) {
    let t21177 = t1234 * t6594;
    let t21182 = t5825 * t1214;
    let t21183 = t5296 * t21182;
    let t21184 = t1042 * t21183;
    let t21188 = t3172 * t6630;
    let t21189 = t3600 * t21188;
    let t21192 = t247 * t3634 * t6425;
    (t21177, t21184, t21189, t21192)
}
