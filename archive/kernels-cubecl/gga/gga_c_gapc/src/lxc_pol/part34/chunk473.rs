//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 473/1427 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk473<F: Float>(t2232: F, t2598: F, t2597: F, t604: F, t924: F, t819: F, t923: F, t181: F, t891: F, t2299: F, t314: F, t298: F) -> (F, F, F, F, F, F, F) {
    let t2599 = t2598 * t2232;
    let t2600 = t2597 * t2599;
    let t2603 = t604 * t924;
    let t2606 = t819 * t923;
    let t2607 = t181 * t2606;
    let t2610 = t819 * t891;
    let t2611 = t181 * t2610;
    let t2614 = t314 * t2299;
    let t2615 = t298 * t2614;
    (t2599, t2600, t2603, t2607, t2611, t2614, t2615)
}
