//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 537/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk537(t235: f64, t7262: f64, t649: f64, t876: f64, t27: f64, t2084: f64, t352: f64, t2145: f64, t3924: f64, t839: f64, t333: f64, t2139: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t7263 = t235 * t7262;
    let t7264 = t649 * t876;
    let t7265 = t27 * t7264;
    let t7266 = t7263 * t7265;
    let t7267 = 0.68186654135613354322e-2_f64 * t7266;
    let t7268 = t2084 * t352;
    let t7269 = t27 * t7268;
    let t7270 = t2145 * t7269;
    let t7273 = t235 * t3924;
    let t7274 = t649 * t839;
    let t7275 = t27 * t7274;
    let t7276 = t7273 * t7275;
    let t7277 = 0.6818665413561335432e-1_f64 * t7276;
    let t7278 = t2084 * t333;
    let t7279 = t27 * t7278;
    let t7280 = t2139 * t7279;
    (t7263, t7265, t7267, t7269, t7270, t7273, t7275, t7277, t7279, t7280)
}
