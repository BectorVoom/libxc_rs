//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 920/1092 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk920<F: Float>(t30546: F, t8657: F, t4198: F, t7646: F, t4452: F, t1061: F, t535: F, t7380: F, t1165: F, t33509: F, t604: F, t7346: F, t1181: F, t2068: F, t23445: F, t30468: F, t4425: F) -> (F, F, F, F, F, F, F) {
    let t34478 = t30546 * t8657;
    let t34481 = t4198 * t7646;
    let t34482 = t34481 * t4452;
    let t34487 = t535 * t1061;
    let t34488 = t7380 * t34487;
    let t34492 = t7346 * t1165 * t604 * t33509;
    let t34497 = t2068 * t1181 * t604 * t23445;
    let t34500 = t30468 * t4425;
    (t34478, t34482, t34487, t34488, t34492, t34497, t34500)
}
