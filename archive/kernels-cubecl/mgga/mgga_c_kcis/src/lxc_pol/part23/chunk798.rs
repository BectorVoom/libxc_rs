//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 798/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk798<F: Float>(t4106: F, t531: F, t3393: F, t4227: F, t1520: F, t752: F, t1466: F, t4243: F, t11824: F, t569: F, t3733: F, t4291: F, sigma2: F) -> (F, F, F, F, F, F, F) {
    let t12417 = t4106 * t531;
    let t12427 = t3393 * t4227;
    let t12431 = t752 * t1520;
    let t12504 = t4243 * t1466;
    let t12505 = t12504 * sigma2;
    let t12520 = t569 * t11824;
    let t12530 = t3733 * t4291;
    (t12417, t12427, t12431, t12504, t12505, t12520, t12530)
}
