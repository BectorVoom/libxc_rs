//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 196/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk196(t1223: f64, t31: f64, t212: f64, t222: f64, t1189: f64, t492: f64, t140: f64, t453: f64, t73: f64, t75: f64, t80: f64, t1007: f64, t78: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t1224 = t31 * t1223;
    let t1227 = 0.21341877202031537856e0_f64 * t212 * t1224 * t222;
    let t1228 = t212 * t1189;
    let t1229 = t1228 * t492;
    let t1231 = t453 * t140;
    let t1279 = t75 * t73;
    let t1281 = 132.0_f64 * t1279 * t80;
    let t1284 = t78 * t1007;
    (t1227, t1228, t1229, t1231, t1281, t1284)
}
