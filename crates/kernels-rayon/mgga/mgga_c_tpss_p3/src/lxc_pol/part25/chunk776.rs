//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 776/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk776(t187: f64, t5343: f64, t4433: f64, t4436: f64, t4439: f64, t1625: f64, t4528: f64, t2281: f64, t2285: f64, t2310: f64, t3182: f64, t3183: f64, t3189: f64, t3194: f64, t3196: f64, t5326: f64, t5327: f64) -> (f64, f64, f64, f64, f64) {
    let t5345 = 0.19751673498613801407e-1_f64 * t5343 * t187;
    let t5346 = 2.0_f64 * t4433;
    let t5347 = 0.36622894612013090108e-3_f64 * t4436;
    let t5348 = 0.11696447245269292414e1_f64 * t4439;
    let t5349 = t4528 * t1625;
    let t5352 = 6.0_f64 * t3183 * t5349 - t2281 - t2285 + t2310 - t3182 + t3189 + t3194 - t3196 - t5326 - t5327 + t5345 + t5346 - t5347 - t5348;
    (t5345, t5346, t5347, t5348, t5352)
}
