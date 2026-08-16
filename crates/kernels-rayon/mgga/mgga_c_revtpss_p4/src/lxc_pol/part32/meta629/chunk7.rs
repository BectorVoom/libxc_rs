//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 2026/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2026(t28368: f64, t99404: f64, t98849: f64, t30405: f64, t689: f64, t25431: f64, t25411: f64, t103400: f64, t103404: f64, t103422: f64, t103424: f64, t106290: f64, t106410: f64, t25391: f64, t26550: f64, t27199: f64, t27349: f64, t27353: f64, t28385: f64, t30337: f64, t62624: f64, t8012: f64, t92917: f64, t99303: f64) -> f64 {
    let t110525 = t99404 * t28368;
    let t110527 = t98849 * t28368;
    let t110541 = t30405 * t689;
    let t110542 = t25431 * t110541;
    let t110544 = t25411 * t110541;
    let t110551 = -0.14634331517634470219e-1_f64 * t103400 - 0.17347256376410398924e1_f64 * t25391 * t26550 * t106410 - 0.14456046980341999104e-1_f64 * t110525 + 0.25702851531048074406e-1_f64 * t110527 + 0.8673628188205199462e0_f64 * t27353 * t26550 * t62624 - 0.8673628188205199462e0_f64 * t25391 * t26550 * t106290 - 0.34270468708064099208e-1_f64 * t103404 + 0.8673628188205199462e0_f64 * t99303 * t8012 + 0.8673628188205199462e0_f64 * t27199 * t28385 + 0.34270468708064099208e-1_f64 * t103422 - 0.72280234901709995518e-2_f64 * t110542 + 0.12851425765524037203e-1_f64 * t110544 - 0.17347256376410398924e1_f64 * t25391 * t103424 * t27349 - 0.17347256376410398924e1_f64 * t92917 * t30337;
    t110551
}
