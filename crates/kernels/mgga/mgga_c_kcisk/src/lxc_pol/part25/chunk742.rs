//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 742/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk742<F: Float>(t9679: F, t9680: F, t1799: F, t1863: F, t2789: F, t415: F, t1871: F, t705: F) -> (F, F, F, F, F) {
    let t9681 = t9679 * t9680;
    let t9682 = t1799 * t9681;
    let t9684 = t1863 * t2789;
    let t9685 = t415 * t9684;
    let t9687 = t705 * t1871;
    (t9681, t9682, t9684, t9685, t9687)
}
