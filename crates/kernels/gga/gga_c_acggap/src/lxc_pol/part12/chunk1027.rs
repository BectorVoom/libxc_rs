//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1027/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1027<F: Float>(t13462: F, t2065: F, t2450: F, t56: F, t1165: F, t4353: F, t604: F, t1581: F, t7614: F, t2327: F, t7780: F, t2068: F, t20935: F, t7351: F) -> (F, F, F, F, F) {
    let t34278 = t2450 * t2065 * t56 * t13462;
    let t34281 = t34278 * t1165 * t604 * t4353;
    let t34284 = t7614 * t1581;
    let t34286 = t7780 * t2327;
    let t34291 = t2068 * t1165 * t7351 * t20935;
    (t34278, t34281, t34284, t34286, t34291)
}
