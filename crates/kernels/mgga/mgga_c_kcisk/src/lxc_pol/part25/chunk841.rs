//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 841/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk841<F: Float>(t12586: F, t2885: F, t172: F, t849: F, t157: F, t2914: F, t2922: F, t119: F, t814: F, t298: F, t831: F, t2850: F, t2880: F, t142: F, t2884: F, t2888: F) -> (F, F, F, F, F, F, F) {
    let t12588 = 6.0 * t2885 * t12586;
    let t12589 = t172 * t849;
    let t12592 = t157 * t2914;
    let t12595 = t157 * t2922;
    let t12598 = t119 * t814;
    let t12601 = 0.71233333333333333334e-1 * t298 * t12598 * t831;
    let t12604 = 0.53425e-1 * t298 * t2850 * t2880;
    let t12605 = t142 * t2884;
    let t12608 = 0.85917146441092277512e0 * t298 * t12605 * t2888;
    (t12588, t12589, t12592, t12595, t12601, t12604, t12608)
}
