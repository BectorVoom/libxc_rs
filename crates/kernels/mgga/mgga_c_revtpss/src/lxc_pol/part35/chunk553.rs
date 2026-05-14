//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 553/1093 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk553<F: Float>(t300: F, t6212: F, t6185: F, t1642: F, t4719: F, t2986: F, t6189: F, t973: F, t981: F, t6205: F, t964: F, t3011: F, t3014: F, t3037: F, t4571: F, t6094: F, t6098: F, t6102: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t6213 = t300 * t6212;
    let t6215 = 0.19751673498613801407e-1 * t300 * t6185;
    let t6217 = 0.11696447245269292414e1 * t4719 * t1642;
    let t6219 = t2986 * t6189 * t973;
    let t6221 = 0.11696447245269292414e1 * t981 * t6219;
    let t6223 = t964 * t6205 * t973;
    let t6225 = 0.5848223622634646207e0 * t981 * t6223;
    let t6226 = t3011 * t6189;
    let t6227 = t6226 * t3014;
    let t6229 = 0.17315859105681463759e2 * t981 * t6227;
    let t6234 = t3037 + 0.55555555555555555556e-2 * t4571 - 0.55555555555555555555e-2 * t6094 + 0.16666666666666666667e-1 * t6098 - 0.83333333333333333333e-2 * t6102;
    (t6213, t6215, t6217, t6219, t6221, t6223, t6225, t6227, t6229, t6234)
}
