//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1973/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1973(t30308: f64, t686: f64, t72: f64, t25895: f64, t25878: f64, t109425: f64, t25899: f64, t30261: f64, t689: f64, t25904: f64, t102205: f64, t102213: f64, t102217: f64, t102219: f64, t102225: f64, t102237: f64, t102239: f64, t27837: f64, t28841: f64, t96246: f64, t96253: f64) -> f64 {
    let t109449 = t30308 * t72 * t686;
    let t109450 = t25895 * t109449;
    let t109453 = t25878 * t109449;
    let t109455 = t25899 * t109425;
    let t109457 = t30261 * t689;
    let t109458 = t25904 * t109457;
    let t109460 = t25899 * t109457;
    let t109467 = 0.91399340044406952588e-2_f64 * t102205 - 0.14456046980341999104e-1_f64 * t109450 + t102213 - t102217 + 0.38549458614245330944e-1_f64 * t102219 + 0.25702851531048074406e-1_f64 * t109453 + 0.12851425765524037203e-1_f64 * t109455 - 0.72280234901709995518e-2_f64 * t109458 + 0.12851425765524037203e-1_f64 * t109460 - 0.68540937416128198419e-2_f64 * t102225 - 0.17135234354032049604e-1_f64 * t96246 - 0.65049603595885220126e-3_f64 * t96253 + 0.17347256376410398924e1_f64 * t27837 * t28841 + t102237 - t102239;
    t109467
}
