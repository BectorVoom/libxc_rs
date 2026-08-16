//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 1057/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk1057<F: Float>(t1181: F, t5116: F, t7351: F, t7564: F, t1350: F, t1992: F, t30147: F, t7586: F, t142: F, t4495: F, t7436: F, t4479: F, t8888: F) -> (F, F, F, F) {
    let t34522 = t7564 * t1181 * t7351 * t5116;
    let t34526 = t30147 * t7586 * t1992 * t1350;
    let t34529 = t7436 * t142 * t4495;
    let t34532 = t8888 * t142 * t4479;
    (t34522, t34526, t34529, t34532)
}
