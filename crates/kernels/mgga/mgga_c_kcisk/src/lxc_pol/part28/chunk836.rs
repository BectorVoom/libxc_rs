//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 836/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk836<F: Float>(t4825: F, t667: F, t10471: F, t140: F, t673: F, t1896: F, t1901: F, t4971: F, t654: F, t4597: F, t642: F, t1870: F, t704: F, t1862: F, t5060: F, t1689: F, t4822: F, sigma2: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t11200 = 1.0 / t4825 / t667;
    let t11208 = t140 * t10471 * t673;
    let t11209 = t11208 * t1896;
    let t11211 = t11208 * t1901;
    let t11213 = t654 * t4971;
    let t11218 = t642 * t4597;
    let t11224 = t1870 * t1870;
    let t11225 = 1.0 / t11224;
    let t11226 = t704 * t11225;
    let t11227 = t11226 * sigma2;
    let t11236 = t1862 * t5060;
    let t11237 = t11236 * sigma2;
    let t11245 = t1689 * t4822;
    (t11200, t11208, t11209, t11211, t11213, t11218, t11225, t11226, t11227, t11236, t11237, t11245)
}
