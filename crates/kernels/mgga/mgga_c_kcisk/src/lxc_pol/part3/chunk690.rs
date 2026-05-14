//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 690/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk690<F: Float>(t10450: F, t1835: F, t10464: F, t1919: F, t5160: F, t965: F, t5163: F, t1842: F, t4726: F, t10488: F, t706: F, t1659: F, t1060: F, t1846: F, t3293: F, t696: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t11580 = t1835 * t10450;
    let t11583 = t1919 * t10464;
    let t11586 = t965 * t5160;
    let t11588 = t965 * t5163;
    let t11590 = t1842 * t10450;
    let t11593 = t4726 * t10464;
    let t11596 = t706 * t10488;
    let t11599 = t1659 * t10488;
    let t11602 = t1835 * t10488;
    let t11605 = t1846 * t1060;
    let t11607 = t696 * t3293;
    (t11580, t11583, t11586, t11588, t11590, t11593, t11596, t11599, t11602, t11605, t11607)
}
