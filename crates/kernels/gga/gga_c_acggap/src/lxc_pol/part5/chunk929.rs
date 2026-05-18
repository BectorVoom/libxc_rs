//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 929/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk929<F: Float>(t14283: F, t425: F, t431: F, t438: F, t3243: F, t390: F, t996: F, t1020: F, t3237: F, t1039: F, t12295: F, t383: F) -> (F, F, F, F, F, F) {
    let t14284 = t14283 * t425;
    let t14286 = t14283 * t431;
    let t14288 = t14283 * t438;
    let t14292 = F::new(0.12004725073059526352e-1) * t3243 * t996 * t390;
    let t14297 = t3237 * t1020;
    let t14301 = F::new(0.25724410870841842184e-2) * t12295 * t383 * t1039;
    (t14284, t14286, t14288, t14292, t14297, t14301)
}
