//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 911/1092 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk911<F: Float>(t1181: F, t5094: F, t7564: F, t8600: F, t31878: F, t4925: F, t1541: F, t31631: F, t13462: F, t2065: F, t2450: F, t56: F, t1165: F, t4353: F, t604: F, t1581: F, t7614: F) -> (F, F, F, F, F, F) {
    let t34269 = t7564 * t1181 * t8600 * t5094;
    let t34271 = t31878 * t4925;
    let t34273 = t31631 * t1541;
    let t34278 = t2450 * t2065 * t56 * t13462;
    let t34281 = t34278 * t1165 * t604 * t4353;
    let t34284 = t7614 * t1581;
    (t34269, t34271, t34273, t34278, t34281, t34284)
}
