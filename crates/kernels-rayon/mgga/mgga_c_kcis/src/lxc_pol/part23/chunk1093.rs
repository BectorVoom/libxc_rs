//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1093/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1093(t28409: f64, t303: f64, t1459: f64, t2012: f64, t1014: f64, t8179: f64, t167: f64, t7909: f64, t16892: f64, t18210: f64, t8158: f64, t2237: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t28410 = t303 * t28409;
    let t28412 = t1459 * t2012;
    let t28413 = t303 * t28412;
    let t28415 = t1014 * t8179;
    let t28419 = t7909 * t167;
    let t28420 = t16892 * t28419;
    let t28423 = t18210 * t8158;
    let t28424 = t2237 * t28423;
    (t28410, t28412, t28413, t28415, t28419, t28420, t28423, t28424)
}
