//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 907/1092 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk907<F: Float>(t1421: F, t1992: F, t30827: F, t7842: F, t1165: F, t4752: F, t7351: F, t7575: F, t2450: F, t7583: F, t8461: F, t1427: F, t1181: F, t4757: F, t7564: F, t4545: F, t604: F) -> (F, F, F, F, F, F) {
    let t34179 = t30827 * t7842 * t1992 * t1421;
    let t34183 = t7575 * t1165 * t7351 * t4752;
    let t34186 = t2450 * t7583 * t8461;
    let t34189 = t34186 * t7842 * t1992 * t1427;
    let t34193 = t7564 * t1181 * t7351 * t4757;
    let t34197 = t7575 * t1181 * t604 * t4545;
    (t34179, t34183, t34186, t34189, t34193, t34197)
}
