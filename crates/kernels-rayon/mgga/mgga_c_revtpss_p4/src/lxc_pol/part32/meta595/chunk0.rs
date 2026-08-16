//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1927/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1927(t14991: f64, t95936: f64, t7407: f64, t99373: f64, t2435: f64, t28390: f64, t102993: f64, t25411: f64, t2470: f64, t28359: f64, t7064: f64, t7997: f64, t822: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t103396 = t95936 * t14991;
    let t103399 = 0.25702851531048074406e-1_f64 * t99373 * t7407;
    let t103400 = t2435 * t28390;
    let t103404 = t25411 * t102993;
    let t103421 = t28359 * t2470;
    let t103422 = t7064 * t103421;
    let t103424 = t822 * t7997;
    (t103396, t103399, t103400, t103404, t103421, t103422, t103424)
}
