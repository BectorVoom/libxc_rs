//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 933/1115 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk933<F: Float>(t2252: F, t6212: F, t1234: F, t19790: F, t560: F, t1593: F, t1554: F, t545: F, t6534: F, t1567: F, t489: F, t146: F, t252: F, t2135: F, t3433: F, t108: F, t2214: F) -> (F, F, F, F, F, F, F, F, F) {
    let t20102 = t6212 * t2252;
    let t20132 = t6212 * t1234;
    let t20146 = t19790 * t560;
    let t20238 = t6212 * t1593;
    let t20294 = t6212 * t1554;
    let t20298 = t545 * t6534;
    let t20303 = t489 * t1567;
    let t20305 = t146 * t20303 * t252;
    let t20339 = t3433 * t2135;
    let t20407 = t2214 * t108;
    (t20102, t20132, t20146, t20238, t20294, t20298, t20305, t20339, t20407)
}
