//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 197/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk197<F: Float>(t125: F, t19: F, t22: F, t326: F, t130: F, t154: F, t37: F, t38: F) -> (F, F, F, F, F, F) {
    let t569 = t125 * t19;
    let t575 = F::cast_from(1.0_f64) / t22 / t326;
    let t576 = t130 * t575;
    let t577 = t154 * t19;
    let t594 = t38 * t37;
    let t595 = F::cast_from(1.0_f64) / t594;
    (t569, t575, t576, t577, t594, t595)
}
