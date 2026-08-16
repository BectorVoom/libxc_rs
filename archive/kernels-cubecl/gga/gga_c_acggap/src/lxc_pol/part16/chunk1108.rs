//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 1108/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk1108<F: Float>(t1992: F, t30692: F, t7842: F, t9587: F, t7839: F, t9601: F, t1181: F, t26757: F, t599: F, t7413: F, t6237: F, t7561: F) -> (F, F, F, F) {
    let t39356 = t30692 * t7842 * t1992 * t9587;
    let t39358 = t7839 * t9601;
    let t39362 = t7413 * t1181 * t599 * t26757;
    let t39364 = t7561 * t6237;
    (t39356, t39358, t39362, t39364)
}
