//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 832/1066 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk832<F: Float>(t7637: F, t7796: F, t1980: F, t1982: F, t1992: F, t5: F, t965: F, t1004: F, t390: F, t7613: F, t1998: F, t3786: F, t151: F, t37: F, t56: F, t593: F, t7508: F) -> (F, F, F, F, F) {
    let t30993 = t7637 * t7796;
    let t30998 = t1980 * t1982 * t5 * t965 * t1992;
    let t31001 = t1004 * t7613 * t390;
    let t31002 = 0.12004725073059526352e-1 * t31001;
    let t31003 = t1998 * t3786;
    let t31009 = t151 * t593 / t7508 / t37 * t56;
    (t30993, t30998, t31002, t31003, t31009)
}
