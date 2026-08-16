//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 2020/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2020(t7407: f64, t99272: f64, t26482: f64, t99404: f64, t98849: f64, t103363: f64, t25305: f64, t14991: f64, t95936: f64, t99373: f64, t2435: f64, t28390: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t103382 = 0.14456046980341999104e-1_f64 * t99272 * t7407;
    let t103391 = 0.14456046980341999104e-1_f64 * t99404 * t26482;
    let t103393 = 0.25702851531048074406e-1_f64 * t98849 * t26482;
    let t103394 = t25305 * t103363;
    let t103396 = t95936 * t14991;
    let t103399 = 0.25702851531048074406e-1_f64 * t99373 * t7407;
    let t103400 = t2435 * t28390;
    (t103382, t103391, t103393, t103394, t103396, t103399, t103400)
}
