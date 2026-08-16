//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1049/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1049<F: Float>(t1181: F, t33751: F, t599: F, t7413: F, t1165: F, t30282: F, t33911: F, t604: F, t1992: F, t5616: F, t7585: F, t7586: F) -> (F, F, F) {
    let t34667 = t7413 * t1181 * t599 * t33751;
    let t34671 = t30282 * t1165 * t604 * t33911;
    let t34675 = t7585 * t7586 * t1992 * t5616;
    (t34667, t34671, t34675)
}
