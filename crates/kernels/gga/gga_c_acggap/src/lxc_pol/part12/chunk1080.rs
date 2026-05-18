//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1080/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1080<F: Float>(t30154: F, t35225: F, t7586: F, t1535: F, t4180: F, t7646: F, t4393: F, t8511: F, t4414: F, t7822: F, t1181: F, t30327: F, t4358: F, t599: F) -> (F, F, F, F, F) {
    let t35227 = t30154 * t7586 * t35225;
    let t35230 = t4180 * t7646 * t1535;
    let t35232 = t8511 * t4393;
    let t35234 = t7822 * t4414;
    let t35238 = t30327 * t1181 * t599 * t4358;
    (t35227, t35230, t35232, t35234, t35238)
}
