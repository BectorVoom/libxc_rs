//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 801/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk801<F: Float>(t2359: F, t806: F, t2670: F, t566: F, t2360: F, t2819: F, t2781: F, t7278: F, t2469: F, t9650: F, t7261: F) -> (F, F, F, F, F, F) {
    let t9907 = t2359 * t806;
    let t9910 = t566 * t2670;
    let t9915 = t2360 * t2819;
    let t9918 = t7278 * t2781;
    let t9921 = t9650 * t2469;
    let t9922 = t7261 * t9921;
    (t9907, t9910, t9915, t9918, t9921, t9922)
}
