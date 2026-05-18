//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 979/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk979<F: Float>(t1350: F, t1992: F, t30147: F, t7586: F, t5129: F, t7647: F, t5133: F, t5101: F, t7741: F, t1434: F, t7746: F, t4680: F, t7426: F, t8476: F) -> (F, F, F, F, F, F) {
    let t34526 = t30147 * t7586 * t1992 * t1350;
    let t34534 = t7647 * t5129;
    let t34537 = t7647 * t5133;
    let t34547 = t7741 * t5101;
    let t34549 = t7746 * t1434;
    let t34556 = t7426 * t4680 * t8476;
    (t34526, t34534, t34537, t34547, t34549, t34556)
}
