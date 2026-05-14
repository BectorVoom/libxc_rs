//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1277/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1277<F: Float>(t2185: F, t23657: F, t4714: F, t590: F, t5900: F, t27147: F, t925: F, t95292: F, t95293: F, t1039: F, t358: F, t3424: F, t105909: F, t1570: F, t3188: F, t105900: F, t1557: F) -> (F, F, F, F, F, F) {
    let t119828 = t23657 * t2185 * t5900 * t4714 * t590;
    let t119832 = t95292 * t95293 * t925 * t27147;
    let t119834 = t1039 * t358;
    let t119837 = t95292 * t95293 * t119834 * t3424;
    let t119842 = t95292 * t105909 * t1039 * t1570 * t3188;
    let t119847 = t95292 * t105900 * t1039 * t1557 * t3188;
    (t119828, t119832, t119834, t119837, t119842, t119847)
}
