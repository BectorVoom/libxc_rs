//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1969/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1969(t114: f64, t109367: f64, t108138: f64, t96187: f64, t96236: f64, t30256: f64, t689: f64, t25904: f64, t102081: f64, t102084: f64, t102086: f64, t102090: f64, t102093: f64, t102096: f64, t102098: f64, t102101: f64, t102104: f64, t102113: f64, t96197: f64) -> (f64, f64, f64) {
    let t115 = 1.0_f64 < t114;
    let t109368 = piecewise3(t115, 0.0_f64, t109367);
    let t109391 = t96187 * t108138;
    let t109393 = t96236 * t108138;
    let t109396 = t30256 * t689;
    let t109397 = t25904 * t109396;
    let t109399 = t102081 - t102084 - t102086 - t102090 + t102093 + t102096 - t102098 + 0.3427046870806409921e-2_f64 * t102101 - t102104 + 0.28912093960683998207e-1_f64 * t109391 - 0.51405703062096148813e-1_f64 * t109393 + 0.73171657588172351096e-2_f64 * t96197 + t102113 - 0.14456046980341999104e-1_f64 * t109397;
    (t109368, t109396, t109399)
}
