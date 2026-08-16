//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 369/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk369(t2243: f64, t541: f64, t303: f64, t2237: f64, t2239: f64, t589: f64, t570: f64, t573: f64, t1395: f64, t585: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t2244 = t541 * t2243;
    let t2245 = t303 * t2244;
    let t2247 = -0.69505208333333333333e-3_f64 * t2237 * t2239 + 0.24872916666666666666e-2_f64 * t2245;
    let t2248 = t2247 * t589;
    let t2249 = t570 * t573;
    let t2251 = t1395 * t585;
    let t2253 = t2249 / 16.0_f64 - t2251 / 128.0_f64;
    (t2244, t2245, t2247, t2248, t2249, t2251, t2253)
}
