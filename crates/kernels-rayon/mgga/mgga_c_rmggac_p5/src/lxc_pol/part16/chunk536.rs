//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 536/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk536(t235: f64, t7262: f64, t2084: f64, t352: f64, t27: f64, t2145: f64, t3924: f64, t333: f64, t2139: f64, t511: f64, t899: f64, t321: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t7263 = t235 * t7262;
    let t7268 = t2084 * t352;
    let t7269 = t27 * t7268;
    let t7270 = t2145 * t7269;
    let t7273 = t235 * t3924;
    let t7278 = t2084 * t333;
    let t7279 = t27 * t7278;
    let t7280 = t2139 * t7279;
    let t7282 = t899 * t511;
    let t7287 = t2084 * t321;
    (t7263, t7269, t7270, t7273, t7279, t7280, t7282, t7287)
}
