//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1127/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1127<F: Float>(t1849: F, t322: F, t1165: F, t7351: F, t7493: F, t5608: F, t7561: F, t1844: F, t604: F, t7346: F, t1181: F, t2068: F, t39164: F) -> (F, F, F, F, F, F) {
    let t39491 = t1849 * t322;
    let t39494 = t7493 * t1165 * t7351 * t39491;
    let t39497 = t7561 * t5608;
    let t39499 = t1844 * t322;
    let t39502 = t7346 * t1165 * t604 * t39499;
    let t39506 = t2068 * t1181 * t604 * t39164;
    (t39491, t39494, t39497, t39499, t39502, t39506)
}
