//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 920/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk920<F: Float>(t14047: F, t3363: F, t1089: F, t175: F, t301: F, t3037: F, t3210: F, t360: F, t368: F, t398: F, t1095: F, t372: F) -> (F, F, F, F) {
    let t14059 = t14047 * t3363;
    let t14072 = t3210 * t1089 * t175 * t3037 * t301;
    let t14081 = t3210 * t398 * t368 * t3037 * t360;
    let t14086 = t3210 * t398 * t1095 * t3037 * t372;
    (t14059, t14072, t14081, t14086)
}
