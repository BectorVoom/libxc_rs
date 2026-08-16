//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3234/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3234(t4401: f64, t606: f64, t61303: f64, t50865: f64, t50868: f64, t14325: f64, t18559: f64, t14369: f64, t4186: f64, t40156: f64, t11084: f64, t2403: f64, t5962: f64, t61292: f64, t61293: f64, t61295: f64, t61297: f64, t61300: f64, t61302: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t61305 = t4401 * t61303 * t606;
    let t61306 = 24.0_f64 * t61305;
    let t61310 = 16.0_f64 * t50865;
    let t61311 = 48.0_f64 * t50868;
    let t61313 = 48.0_f64 * t14325 * t18559;
    let t61315 = t4401 * t14369 * t4186;
    let t61316 = 48.0_f64 * t61315;
    let t61317 = 0.17315859105681463759e2_f64 * t40156;
    let t61318 = -3.0_f64 * t11084 * t2403 * t5962 - t61292 - t61293 - t61295 - t61297 + t61300 + t61302 + t61306 + t61310 + t61311 + t61313 + t61316 - t61317;
    (t61306, t61310, t61311, t61313, t61316, t61317, t61318)
}
