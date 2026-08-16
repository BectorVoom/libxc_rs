//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1016/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1016<F: Float>(t1181: F, t23736: F, t7351: F, t7575: F, t30268: F, t8956: F, t21099: F, t599: F, t7337: F, t2068: F, t20935: F, t604: F) -> (F, F, F, F) {
    let t34105 = t7575 * t1181 * t7351 * t23736;
    let t34107 = t30268 * t8956;
    let t34111 = t7337 * t1181 * t599 * t21099;
    let t34115 = t2068 * t1181 * t604 * t20935;
    (t34105, t34107, t34111, t34115)
}
