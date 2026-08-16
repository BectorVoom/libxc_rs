//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 977/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk977<F: Float>(t30546: F, t8657: F, t4198: F, t7646: F, t30601: F, t30605: F, t1061: F, t535: F, t7380: F, t1165: F, t33509: F, t604: F, t7346: F) -> (F, F, F, F, F, F, F) {
    let t34478 = t30546 * t8657;
    let t34481 = t4198 * t7646;
    let t34484 = t30601 / F::cast_from(64.0_f64);
    let t34485 = t30605 / F::cast_from(192.0_f64);
    let t34487 = t535 * t1061;
    let t34488 = t7380 * t34487;
    let t34492 = t7346 * t1165 * t604 * t33509;
    (t34478, t34481, t34484, t34485, t34487, t34488, t34492)
}
