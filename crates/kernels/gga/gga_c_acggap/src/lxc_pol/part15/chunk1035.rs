//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1035/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1035<F: Float>(t1782: F, t1992: F, t2095: F, t7426: F, t8480: F, t8605: F, t4680: F, t7564: F, t9607: F, t1181: F, t5819: F, t7351: F, t5814: F, t1854: F, t30148: F, t30159: F, t7586: F) -> (F, F, F, F, F, F) {
    let t40546 = t2095 * t1992 * t1782;
    let t40549 = t7426 * t8480 * t8605;
    let t40553 = t7564 * t4680 * t9607;
    let t40557 = t7564 * t1181 * t7351 * t5819;
    let t40561 = t7564 * t1181 * t7351 * t5814;
    let t40565 = t30159 * t7586 * t30148 * t1854;
    (t40546, t40549, t40553, t40557, t40561, t40565)
}
