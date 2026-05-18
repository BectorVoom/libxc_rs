//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 460/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk460<F: Float>(t108: F, t7212: F, t28: F, t1308: F, t1337: F, t110: F, t1871: F, t7165: F, t1307: F, t1339: F, t452: F, t1332: F) -> (F, F, F, F, F, F, F) {
    let t7213 = t7212 * t108;
    let t7214 = t28 * t7213;
    let t7217 = t1308 * t1337;
    let t7218 = t28 * t7217;
    let t7222 = t1871 * t110 * t7165;
    let t7226 = t452 * t1339 * t1307;
    let t7229 = t1307 * t1332;
    (t7213, t7214, t7217, t7218, t7222, t7226, t7229)
}
