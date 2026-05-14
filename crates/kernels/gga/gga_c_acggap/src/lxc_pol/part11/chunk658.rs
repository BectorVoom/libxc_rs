//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 658/1066 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk658<F: Float>(t137: F, t879: F, t1089: F, t1095: F, t2079: F, t7458: F, t7459: F, t7457: F, t1967: F, t2104: F, t1035: F, t597: F, t864: F, t121: F, t163: F, t1171: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t7542 = t137 * t879;
    let t7544 = t1089 * t1095 * t7542;
    let t7545 = t2079 * t7544;
    let t7546 = 0.15724046144802076034e-3 * t7545;
    let t7548 = t7458 * t1095 * t7459;
    let t7549 = t7457 * t7548;
    let t7550 = 0.20965394859736101378e-3 * t7549;
    let t7551 = t1967 * t2104;
    let t7553 = t1035 * t597;
    let t7554 = t137 * t864;
    let t7556 = t1089 * t1095 * t7554;
    let t7557 = t7553 * t7556;
    let t7558 = 0.31448092289604152068e-3 * t7557;
    let t7559 = t121 * t163;
    let t7560 = t7559 * t1171;
    (t7542, t7544, t7546, t7548, t7550, t7551, t7553, t7554, t7556, t7558, t7559, t7560)
}
