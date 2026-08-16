//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 550/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk550(t219: f64, t2352: f64, t73: f64, t799: f64, t2116: f64, t2133: f64, t778: f64, t222: f64, t224: f64, t776: f64, t779: f64) -> (f64, f64, f64, f64) {
    let t2353 = t2352 * t219;
    let t2357 = t73 * t799;
    let t2358 = t2357 * t2116;
    let t2361 = t778 * t2133;
    let t2364 = -12.0_f64 * t222 * t2358 + 3.0_f64 * t222 * t2361 - t224 * t2353 + 6.0_f64 * t776 * t779;
    (t2353, t2358, t2361, t2364)
}
