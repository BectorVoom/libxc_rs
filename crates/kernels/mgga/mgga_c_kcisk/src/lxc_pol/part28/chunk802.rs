//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 802/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk802<F: Float>(t20: F, t2447: F, t654: F, t1693: F, t2454: F, t648: F, t2364: F, t9665: F, t1775: F) -> (F, F, F, F, F, F) {
    let t9926 = t2447 * t654 * t20;
    let t9927 = t1693 * t9926;
    let t9931 = t648 * t2454 * t20;
    let t9932 = t1693 * t9931;
    let t9935 = t9665 * t2364;
    let t9936 = t1775 * t9935;
    (t9926, t9927, t9931, t9932, t9935, t9936)
}
