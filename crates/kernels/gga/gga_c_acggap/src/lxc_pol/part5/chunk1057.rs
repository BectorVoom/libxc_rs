//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1057/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1057<F: Float>(t1165: F, t14373: F, t1532: F, t301: F, t4183: F, t12813: F, t5286: F, t1163: F, t1181: F, t14575: F, t535: F, t1552: F, t3451: F, t372: F) -> (F, F, F, F) {
    let t18605 = t14373 * t1165 * t1532 * t4183 * t301;
    let t18607 = t12813 * t5286;
    let t18611 = t1163 * t1181 * t535 * t14575;
    let t18616 = t3451 * t1165 * t1552 * t4183 * t372;
    (t18605, t18607, t18611, t18616)
}
