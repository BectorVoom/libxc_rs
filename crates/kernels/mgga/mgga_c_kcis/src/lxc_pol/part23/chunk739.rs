//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 739/1177 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk739<F: Float>(t10138: F, t534: F, t333: F, t3754: F, t740: F, t2642: F, t113: F, t11425: F, t11966: F, t518: F, t1405: F, t1441: F, t1420: F, t4016: F, t4031: F, t532: F) -> (F, F, F, F, F, F, F, F) {
    let t12062 = t10138 * t534;
    let t12064 = 0.72818958333333333333e-4 * t333 * t12062;
    let t12065 = t740 * t3754;
    let t12066 = t12065 * t2642;
    let t12070 = t113 * t11425;
    let t12084 = 0.14055920378328537299e-1 * t11966 * t518;
    let t12085 = t1441 * t1405;
    let t12087 = t4016 * t1420;
    let t12089 = t532 * t4031;
    (t12064, t12065, t12066, t12070, t12084, t12085, t12087, t12089)
}
