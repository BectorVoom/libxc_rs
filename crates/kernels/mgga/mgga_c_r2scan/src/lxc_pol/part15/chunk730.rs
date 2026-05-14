//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 730/1115 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk730<F: Float>(t2049: F, t864: F, t2287: F, t244: F, t6007: F, t2279: F, t2292: F, t2288: F, t357: F, t761: F, t366: F, t2281: F, t757: F, t2289: F, t2300: F, t862: F) -> (F, F, F, F, F, F, F, F, F) {
    let t6813 = t864 * t2049;
    let t6817 = 1.0 / t2287 / t244;
    let t6818 = t6817 * t6007;
    let t6821 = t2279 * t2292;
    let t6826 = t2288 * t2049;
    let t6827 = t761 * t357;
    let t6828 = t6827 * t366;
    let t6831 = t2281 * t2292;
    let t6835 = 1.0 / t2287 / t757;
    let t6836 = t6835 * t6007;
    let t6839 = t2289 * t2292;
    let t6842 = t357 * t2300;
    let t6843 = t862 * t6842;
    (t6813, t6818, t6821, t6826, t6828, t6831, t6836, t6839, t6843)
}
