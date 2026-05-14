//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 923/1092 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk923<F: Float>(t2001: F, t5096: F, t5101: F, t7741: F, t1434: F, t7746: F, t1181: F, t4526: F, t7351: F, t7564: F, t4680: F, t7426: F, t8476: F, t7575: F, t8445: F, t30937: F, t8450: F) -> (F, F, F, F, F, F, F) {
    let t34545 = t2001 * t5096;
    let t34547 = t7741 * t5101;
    let t34549 = t7746 * t1434;
    let t34553 = t7564 * t1181 * t7351 * t4526;
    let t34556 = t7426 * t4680 * t8476;
    let t34559 = t7575 * t4680 * t8445;
    let t34561 = t30937 * t8450;
    (t34545, t34547, t34549, t34553, t34556, t34559, t34561)
}
