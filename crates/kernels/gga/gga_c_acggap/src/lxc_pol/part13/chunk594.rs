//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 594/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk594<F: Float>(t1140: F, t1526: F, t509: F, t987: F, t1165: F, t1532: F, t4162: F, t1163: F, t530: F, t945: F, t535: F, t1181: F) -> (F, F, F, F, F, F) {
    let t4368 = F::new(7.0) / F::new(144.0) * t1140 * t1526;
    let t4369 = t987 * t509;
    let t4372 = t1165 * t1532 * t4162;
    let t4373 = t1163 * t4372;
    let t4376 = t1165 * t530 * t945;
    let t4379 = t535 * t945;
    let t4380 = t1181 * t4379;
    (t4368, t4369, t4372, t4373, t4376, t4380)
}
