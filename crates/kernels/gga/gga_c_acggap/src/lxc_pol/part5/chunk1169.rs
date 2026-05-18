//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1169/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1169<F: Float>(t513: F, t930: F, t1165: F, t1532: F, t3194: F, t322: F, t6258: F, t1748: F, t879: F, t1444: F, t1181: F, t4282: F, t530: F) -> (F, F, F, F, F) {
    let t21128 = t930 * t513;
    let t21136 = t3194 * t1165 * t1532 * t6258 * t322;
    let t21141 = t3194 * t1165 * t1532 * t1748 * t879;
    let t21143 = t1444 * t322;
    let t21146 = t4282 * t1181 * t530 * t21143;
    (t21128, t21136, t21141, t21143, t21146)
}
