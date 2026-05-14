//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 634/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk634<F: Float>(t1802: F, t6719: F, t1799: F, t1864: F, t2533: F, t415: F, t2477: F, t696: F, t1060: F, t1814: F, t2063: F, t1824: F, t220: F, t682: F, t1806: F, t2488: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t6720 = t6719 * t1802;
    let t6721 = t1799 * t6720;
    let t6724 = t1864 * t2533;
    let t6725 = t415 * t6724;
    let t6729 = t696 * t2477;
    let t6731 = t2477 * t1060;
    let t6734 = t1814 * t2063;
    let t6735 = t6734 * t1824;
    let t6738 = t682 * t220;
    let t6741 = t1806 * t2488;
    (t6720, t6721, t6724, t6725, t6729, t6731, t6734, t6735, t6738, t6741)
}
