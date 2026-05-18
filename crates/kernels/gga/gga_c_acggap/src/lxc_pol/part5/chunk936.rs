//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 936/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk936<F: Float>(t12730: F, t180: F, t3037: F, t407: F, t1160: F, t3065: F, t955: F, t3073: F, t945: F, t1237: F, t13259: F, t3066: F, t3077: F) -> (F, F, F, F, F) {
    let t14525 = t12730 * t180 * t3037 * t407;
    let t14528 = t1160 * t3065 * t955;
    let t14534 = t3073 * t3065 * t945;
    let t14539 = t13259 * t1237;
    let t14547 = t3077 * t3066;
    (t14525, t14528, t14534, t14539, t14547)
}
