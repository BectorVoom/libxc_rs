//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 649/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk649<F: Float>(t2100: F, t7538: F, t137: F, t879: F, t1089: F, t1095: F, t2079: F, t7458: F, t7459: F, t7457: F, t1967: F, t2104: F, t1035: F, t597: F, t864: F, t121: F, t163: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t7539 = t7538 * t2100;
    let t7542 = t137 * t879;
    let t7544 = t1089 * t1095 * t7542;
    let t7545 = t2079 * t7544;
    let t7548 = t7458 * t1095 * t7459;
    let t7549 = t7457 * t7548;
    let t7551 = t1967 * t2104;
    let t7553 = t1035 * t597;
    let t7554 = t137 * t864;
    let t7556 = t1089 * t1095 * t7554;
    let t7557 = t7553 * t7556;
    let t7559 = t121 * t163;
    (t7539, t7542, t7544, t7545, t7548, t7549, t7551, t7553, t7554, t7556, t7557, t7559)
}
