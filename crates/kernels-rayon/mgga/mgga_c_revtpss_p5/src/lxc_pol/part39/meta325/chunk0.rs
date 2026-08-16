//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 1105/1507 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1105(t159: f64, t3617: f64, t409: f64, t416: f64, t406: f64, t12295: f64, t11335: f64, t281: f64, t414: f64, t1126: f64, t3383: f64, t1160: f64, t3444: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t12305 = t159 * t3617;
    let t12327 = 1.0_f64 / t409 / t416 / 4.0_f64;
    let t12331 = 1.0_f64/pow_3_2(t406);
    let t12349 = 0.93011851851851851854e0_f64 * t12295;
    let t12351 = t281 * t11335 * t414;
    let t12352 = 0.36514074074074074075e0_f64 * t12351;
    let t12361 = t1126 * t3383;
    let t12367 = 0.28842592592592592592e-1_f64 * t12295;
    let t12382 = 0.55403703703703703703e-1_f64 * t12295;
    let t12397 = 0.53272592592592592592e-1_f64 * t12295;
    let t12418 = t3444 * t1160;
    (t12305, t12327, t12331, t12349, t12351, t12352, t12361, t12367, t12382, t12397, t12418)
}
