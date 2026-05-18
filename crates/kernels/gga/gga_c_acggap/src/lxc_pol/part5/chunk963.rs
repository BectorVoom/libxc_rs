//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 963/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk963<F: Float>(t1181: F, t15407: F, t3456: F, t535: F, t1165: F, t14575: F, t3194: F, t530: F, t14050: F, t4971: F, t3379: F, t4975: F) -> (F, F, F, F) {
    let t15410 = t3456 * t1181 * t535 * t15407;
    let t15429 = t3194 * t1165 * t530 * t14575;
    let t15431 = t14050 * t4971;
    let t15469 = t3379 * t4975;
    (t15410, t15429, t15431, t15469)
}
