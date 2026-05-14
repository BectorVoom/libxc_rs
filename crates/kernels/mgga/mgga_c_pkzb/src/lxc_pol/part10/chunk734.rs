//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 734/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk734<F: Float>(t20: F, t398: F, t983: F, t1435: F, t3318: F, t27: F, t23: F, t28: F, t3315: F, t3319: F, t7: F, t980: F, t984: F, sigma2: F) -> (F, F, F, F, F, F) {
    let t3323 = 1.0 / t20 / t398;
    let t3324 = sigma2 * t3323;
    let t3329 = t983 * t983;
    let t3330 = t1435 * t3329;
    let t3333 = -t3318;
    let t3334 = t27 * t3333;
    let t3337 = 10.0 / 9.0 * t7 * t3315 + 5.0 / 3.0 * t7 * t3319 + 88.0 / 9.0 * t3324 * t28 - 80.0 / 9.0 * t980 * t984 + 10.0 / 9.0 * t23 * t3330 + 5.0 / 3.0 * t23 * t3334;
    (t3324, t3329, t3330, t3333, t3334, t3337)
}
