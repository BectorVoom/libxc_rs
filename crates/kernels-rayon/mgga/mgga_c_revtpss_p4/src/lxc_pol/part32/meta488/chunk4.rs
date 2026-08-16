//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1741/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1741(t225: f64, t28340: f64, t7997: f64, t886: f64, t7071: f64, t27216: f64, t7407: f64, t213: f64, t25383: f64, t257: f64, t26437: f64, t26439: f64, t26448: f64, t26483: f64, t26486: f64, t28310: f64, t28315: f64, t28317: f64, t4534: f64, t7070: f64, t7403: f64, t7424: f64, t7766: f64, t8007: f64) -> (f64, f64, f64, f64, f64) {
    let t28341 = t28340 * t225;
    let t28347 = t7997 * t886;
    let t28348 = t7071 * t28347;
    let t28352 = t27216 * t7407;
    let t28358 = -t26437 + t26439 - 0.54878743191129263322e-2_f64 * t26448 + 0.8673628188205199462e0_f64 * t7070 * t28310 - 0.14456046980341999104e-1_f64 * t28315 + 0.25702851531048074406e-1_f64 * t28317 + 0.12851425765524037203e-1_f64 * t26483 + 0.65854491829355115987e0_f64 * t213 * t28341 * t257 + 0.8673628188205199462e0_f64 * t25383 * t8007 + 0.8673628188205199462e0_f64 * t7070 * t28348 + 0.25702851531048074406e-1_f64 * t26486 - 0.12851425765524037203e-1_f64 * t28352 - 0.4336814094102599731e0_f64 * t7766 * t7424 - 0.65854491829355115987e0_f64 * t7403 * t4534;
    (t28341, t28347, t28348, t28352, t28358)
}
