//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2128/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2128(t105945: f64, t7063: f64, t7060: f64, t29637: f64, t786: f64, t789: f64, t27199: f64, t27317: f64, t27322: f64, t7775: f64, t93306: f64, t93324: f64, t99303: f64, t99391: f64, t99406: f64, t99412: f64, t99420: f64, t99423: f64, t99425: f64, t99435: f64) -> f64 {
    let t106387 = t7063 * t105945;
    let t106388 = t106387 * t7060;
    let t106395 = t786 * t29637 * t789;
    let t106403 = -t99391 - t99406 + 0.38549458614245330944e-1_f64 * t99412 + 0.17135234354032049604e-1_f64 * t93306 - 0.12851425765524037203e-1_f64 * t106388 - t99420 + 0.96373646535613327359e-3_f64 * t99423 - 0.45699670022203476294e-2_f64 * t99425 + 0.8673628188205199462e0_f64 * t99303 * t7775 + 0.9757440539382783019e-2_f64 * t106395 + 0.23131639038696784278e-2_f64 * t99435 + 0.17135234354032049604e-1_f64 * t93324 + 0.17347256376410398924e1_f64 * t27199 * t27322 + 0.17347256376410398924e1_f64 * t27199 * t27317;
    t106403
}
