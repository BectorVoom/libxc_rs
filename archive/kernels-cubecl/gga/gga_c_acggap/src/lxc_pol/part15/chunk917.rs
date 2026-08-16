//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 917/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk917<F: Float>(t30402: F, t31309: F, t407: F, t7325: F, t30409: F, t30418: F, t30546: F, t7428: F, t30374: F, t7570: F, t30394: F, t7323: F, t7326: F) -> (F, F, F, F, F) {
    let t31312 = t31309 * t30402 * t7325 * t407;
    let t31316 = t31309 * t30418 * t30409 * t407;
    let t31318 = t30546 * t7428;
    let t31322 = t30374 * t7570;
    let t31340 = t30394 * t7323 * t7326;
    (t31312, t31316, t31318, t31322, t31340)
}
