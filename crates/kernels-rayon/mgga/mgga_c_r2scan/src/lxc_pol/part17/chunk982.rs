//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 982/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk982(t12056: f64, t3262: f64, t3264: f64, t11559: f64, t3472: f64, t3275: f64, t3787: f64, t860: f64, t10653: f64, t10660: f64, t11357: f64, t11566: f64, t11570: f64, t11574: f64, t12026: f64, t12028: f64, t12031: f64, t12035: f64, t12038: f64, t12041: f64, t12044: f64, t12047: f64, t12050: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t12058 = t3262 * t12056 * t3264;
    let t12059 = 3.0_f64 / 4.0_f64 * t12058;
    let t12060 = t3472 * t11559;
    let t12061 = t3275 * t12060;
    let t12062 = 5.0_f64 / 16.0_f64 * t12061;
    let t12063 = t860 * t3787;
    let t12069 = -t12026 + t12028 - 0.30487649791575028312e-3_f64 * t11566 + 0.43368970657079495308e-4_f64 * t11570 - t12031 + t12035 + t12038 - t12041 - t12044 + 0.30487649791575028312e-3_f64 * t11574 + 0.72042316457491791901e-3_f64 * t10653 + t12047 - t11357 - 0.30487649791575028312e-3_f64 * t10660 + t12050;
    (t12058, t12059, t12060, t12061, t12062, t12063, t12069)
}
