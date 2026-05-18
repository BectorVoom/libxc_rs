//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 1133/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk1133<F: Float>(t31382: F, t31386: F, t31346: F, t4912: F, t7413: F, t7835: F, t8480: F, t30219: F, t8446: F, t1439: F, t30148: F, t30154: F, t7842: F) -> (F, F, F, F, F, F) {
    let t35562 = F::new(13.0) / F::new(48.0) * t31382;
    let t35563 = F::new(0.85748036236139473944e-3) * t31386;
    let t35564 = t31346 * t4912;
    let t35567 = t7413 * t8480 * t7835;
    let t35569 = t30219 * t8446;
    let t35570 = F::new(0.31448092289604152068e-2) * t35569;
    let t35573 = t30154 * t7842 * t30148 * t1439;
    (t35562, t35563, t35564, t35567, t35570, t35573)
}
