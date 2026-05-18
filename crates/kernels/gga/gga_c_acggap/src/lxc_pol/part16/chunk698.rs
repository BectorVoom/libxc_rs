//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 698/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk698<F: Float>(t2100: F, t7538: F, t137: F, t879: F, t1089: F, t1095: F, t2079: F, t7458: F, t7459: F, t7457: F, t1967: F, t2104: F) -> (F, F, F, F, F, F, F) {
    let t7539 = t7538 * t2100;
    let t7540 = F::new(0.31448092289604152068e-3) * t7539;
    let t7542 = t137 * t879;
    let t7544 = t1089 * t1095 * t7542;
    let t7545 = t2079 * t7544;
    let t7546 = F::new(0.15724046144802076034e-3) * t7545;
    let t7548 = t7458 * t1095 * t7459;
    let t7549 = t7457 * t7548;
    let t7550 = F::new(0.20965394859736101378e-3) * t7549;
    let t7551 = t1967 * t2104;
    (t7540, t7542, t7544, t7546, t7548, t7550, t7551)
}
