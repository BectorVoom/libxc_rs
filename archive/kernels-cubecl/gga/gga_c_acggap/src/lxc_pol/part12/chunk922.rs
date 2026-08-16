//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 922/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk922<F: Float>(t1: F, t203: F, t3157: F, t174: F, t31009: F, t31010: F, t172: F, t420: F, t435: F, t7746: F, t993: F, t1131: F, t355: F) -> (F, F, F, F) {
    let t31013 = t3157 * t1 * t203;
    let t31015 = t31009 * t31010 * t174 * t31013;
    let t31020 = t31009 * t420 * t172 * t435 * t31013;
    let t31022 = t7746 * t993;
    let t31024 = t355 * t1131;
    (t31015, t31020, t31022, t31024)
}
