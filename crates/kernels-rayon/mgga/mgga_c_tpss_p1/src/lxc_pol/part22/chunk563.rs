//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 563/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk563(t821: f64, t823: f64, t198: f64, t207: f64, t2208: f64, t2217: f64, t2220: f64, t2242: f64, t2244: f64, t2246: f64, t2292: f64, t2302: f64, t2310: f64, t2333: f64, t2347: f64, t2433: f64, t2436: f64, t2439: f64, t750: f64) -> (f64, f64) {
    let t2440 = t821 * t823;
    let t2444 = -t198 * t207 * t2433 * t2436 + 6.0_f64 * t2439 * t2440 * t750 - t2208 - t2217 - t2220 + t2242 + t2244 + t2246 - t2292 + t2302 + t2310 + t2333 + t2347;
    (t2440, t2444)
}
