//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1015/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1015<F: Float>(t1165: F, t39743: F, t604: F, t7346: F, t39753: F, t7337: F, t31142: F, t9727: F, t2060: F, t361: F, t9733: F, t7450: F, t9659: F, t13287: F, t31195: F, t38861: F) -> (F, F, F, F, F, F) {
    let t40076 = t7346 * t1165 * t604 * t39743;
    let t40080 = t7337 * t1165 * t604 * t39753;
    let t40083 = t31142 * t9727;
    let t40086 = t2060 * t361 * t9733;
    let t40089 = t7450 * t361 * t9659;
    let t40092 = t31195 * t13287 * t38861;
    (t40076, t40080, t40083, t40086, t40089, t40092)
}
