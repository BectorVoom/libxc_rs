//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 916/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk916<F: Float>(t12056: F, t3262: F, t3264: F, t11559: F, t3472: F, t3275: F, t3787: F, t860: F, t10653: F, t10660: F, t11357: F, t11566: F, t11570: F, t11574: F, t12026: F, t12028: F, t12031: F, t12035: F, t12038: F, t12041: F, t12044: F, t12047: F, t12050: F) -> (F, F, F, F, F, F, F) {
    let t12058 = t3262 * t12056 * t3264;
    let t12059 = 3.0 / 4.0 * t12058;
    let t12060 = t3472 * t11559;
    let t12061 = t3275 * t12060;
    let t12062 = 5.0 / 16.0 * t12061;
    let t12063 = t860 * t3787;
    let t12069 = -t12026 + t12028 - 0.30487649791575028312e-3 * t11566 + 0.43368970657079495308e-4 * t11570 - t12031 + t12035 + t12038 - t12041 - t12044 + 0.30487649791575028312e-3 * t11574 + 0.72042316457491791901e-3 * t10653 + t12047 - t11357 - 0.30487649791575028312e-3 * t10660 + t12050;
    (t12058, t12059, t12060, t12061, t12062, t12063, t12069)
}
