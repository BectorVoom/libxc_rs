//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 624/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk624(t1398: f64, t2436: f64, t1364: f64, t2440: f64, t1692: f64, t198: f64, t207: f64, t2208: f64, t2217: f64, t2245: f64, t2292: f64, t2302: f64, t2310: f64, t2333: f64, t2347: f64, t2439: f64, t3594: f64, t3610: f64, t3644: f64, t3646: f64, t3647: f64, t3724: f64, t740: f64, t821: f64, t823: f64) -> (f64, f64) {
    let t3728 = t1398 * t2436;
    let t3731 = t2440 * t1364;
    let t3734 = t198 * t207 * t3724 * t823 - t1692 * t3728 * t821 + 3.0_f64 * t198 * t3610 * t740 + 3.0_f64 * t2439 * t3731 - t2208 - t2217 + t2245 - t2292 + t2302 + t2310 + t2333 + t2347 - t3594 + t3644 + t3646 + t3647;
    (t3728, t3734)
}
