//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 912/1092 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk912<F: Float>(t2327: F, t7780: F, t1165: F, t2068: F, t20935: F, t7351: F, t30318: F, t532: F, t1569: F, t7614: F, t1988: F, t8838: F, t1089: F, t1459: F, t33878: F, t598: F) -> (F, F, F, F, F, F) {
    let t34286 = t7780 * t2327;
    let t34291 = t2068 * t1165 * t7351 * t20935;
    let t34293 = t30318 * t532;
    let t34295 = t7614 * t1569;
    let t34297 = t1988 * t8838;
    let t34301 = t598 * t1089 * t1459 * t33878;
    (t34286, t34291, t34293, t34295, t34297, t34301)
}
