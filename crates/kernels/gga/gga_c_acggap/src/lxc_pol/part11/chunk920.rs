//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 920/1066 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk920<F: Float>(t1427: F, t1992: F, t34186: F, t7842: F, t1181: F, t4757: F, t7351: F, t7564: F, t4545: F, t604: F, t7575: F, t1165: F, t4550: F, t1530: F, t1535: F, t30539: F) -> (F, F, F, F, F) {
    let t34189 = t34186 * t7842 * t1992 * t1427;
    let t34193 = t7564 * t1181 * t7351 * t4757;
    let t34197 = t7575 * t1181 * t604 * t4545;
    let t34201 = t7575 * t1165 * t7351 * t4550;
    let t34204 = t1530 * t30539 * t1535;
    (t34189, t34193, t34197, t34201, t34204)
}
