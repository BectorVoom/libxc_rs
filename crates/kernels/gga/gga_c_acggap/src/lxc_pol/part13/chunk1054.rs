//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 1054/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk1054<F: Float>(t4198: F, t7646: F, t4452: F, t30601: F, t30605: F, t1061: F, t535: F, t7380: F, t1165: F, t33509: F, t604: F, t7346: F) -> (F, F, F, F, F, F) {
    let t34481 = t4198 * t7646;
    let t34482 = t34481 * t4452;
    let t34484 = t30601 / F::new(64.0);
    let t34485 = t30605 / F::new(192.0);
    let t34487 = t535 * t1061;
    let t34488 = t7380 * t34487;
    let t34489 = F::new(0.4584375e-1) * t34488;
    let t34492 = t7346 * t1165 * t604 * t33509;
    (t34482, t34484, t34485, t34487, t34489, t34492)
}
