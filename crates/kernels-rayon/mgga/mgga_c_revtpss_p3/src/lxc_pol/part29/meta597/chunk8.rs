//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 2023/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2023(t103438: f64, t25372: f64, t98801: f64, t25386: f64, t2471: f64, t28373: f64, t103352: f64, t103422: f64, t103424: f64, t103432: f64, t103435: f64, t103437: f64, t1956: f64, t1957: f64, t233: f64, t25383: f64, t25391: f64, t25394: f64, t26493: f64, t26550: f64, t27199: f64, t27353: f64, t28411: f64, t51436: f64, t95872: f64, t95876: f64) -> f64 {
    let t103441 = 0.14456046980341999104e-1_f64 * t25372 * t103438 * t98801;
    let t103444 = 0.25702851531048074406e-1_f64 * t25386 * t103438 * t98801;
    let t103449 = t28373 * t2471;
    let t103451 = -0.4336814094102599731e0_f64 * t1956 * t1957 * t233 * t103352 - 0.52041769129231196772e1_f64 * t25383 * t28411 + 0.17135234354032049604e-1_f64 * t103422 - 0.17347256376410398924e1_f64 * t25391 * t103424 * t25394 + 0.8673628188205199462e0_f64 * t27353 * t26550 * t51436 - 0.34270468708064099208e-1_f64 * t103432 + t103435 - t103437 - t103441 + t103444 + 0.14456046980341999104e-1_f64 * t95872 + 0.17347256376410398924e1_f64 * t27199 * t26493 + 0.72280234901709995518e-2_f64 * t95876 - 0.13009920719177044025e-1_f64 * t103449;
    t103451
}
