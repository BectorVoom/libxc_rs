//! GGA_C_GAPC lxc pol — lxc_pol part 22 (v4rho2sigma2_1) CSE chunk 794/1426 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part22_v4rho2sigma2_1_chunk794<F: Float>(t3109: F, t9120: F, t1404: F, t1720: F, t3108: F, t5553: F, t8687: F, t19: F, t8768: F, t611: F, t3085: F, t3160: F, t608: F) -> (F, F, F, F, F, F) {
    let t9121 = t9120 * t3109;
    let t9123 = t1720 * t1404;
    let t9124 = t3108 * t9123;
    let t9126 = t5553 * t8687;
    let t9128 = t8768 * t19;
    let t9129 = t611 * t9128;
    let t9130 = t9129 * t3085;
    let t9132 = t3160 * t608;
    (t9121, t9124, t9126, t9128, t9130, t9132)
}
