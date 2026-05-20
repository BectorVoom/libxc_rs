//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 916/1234 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk916<F: Float>(t23279: F, t2477: F, t828: F, t23177: F, t827: F, t23245: F, t18426: F, t2747: F, t6035: F, t4364: F, t4365: F, t6017: F) -> (F, F, F, F, F) {
    let t23281 = t2477 * t828 * t23279;
    let t23285 = t827 * t828 * t23177;
    let t23289 = t827 * t828 * t23245;
    let t23293 = t2747 * t18426 * t6035;
    let t23297 = t4364 * t4365 * t6017;
    (t23281, t23285, t23289, t23293, t23297)
}
