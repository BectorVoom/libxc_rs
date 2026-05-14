//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 867/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk867<F: Float>(t1308: F, t4154: F, t1323: F, t164: F, t1309: F, t3966: F, t3984: F, t25: F, t3989: F, t122: F, t4000: F, t389: F, t3970: F, t3962: F, t1318: F, t1293: F, t3969: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t13795 = t4154 * t1308;
    let t13804 = t164 * t1323;
    let t13805 = t1309 * t13804;
    let t13807 = t3966 * t3984;
    let t13809 = t25 * t3989;
    let t13810 = t1309 * t13809;
    let t13820 = t4000 * t122;
    let t13821 = t389 * t13820;
    let t13824 = t3970 * t3984;
    let t13826 = t25 * t3962;
    let t13827 = t1309 * t13826;
    let t13829 = t1318 * t1318;
    let t13830 = 1.0 / t13829;
    let t13839 = t1293 * t3969;
    (t13795, t13805, t13807, t13810, t13820, t13821, t13824, t13827, t13830, t13839)
}
