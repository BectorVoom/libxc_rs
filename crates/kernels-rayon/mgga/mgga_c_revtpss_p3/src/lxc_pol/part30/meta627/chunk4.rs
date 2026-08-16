//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2178/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2178(t7064: f64, t99321: f64, t25411: f64, t99389: f64, t93369: f64, t93372: f64, t93375: f64, t93378: f64, t93382: f64, t93384: f64, t99472: f64, t99475: f64, t99480: f64, t99481: f64, t99485: f64, t99487: f64) -> f64 {
    let t99491 = 0.25702851531048074406e-1_f64 * t7064 * t99321;
    let t99493 = 0.25702851531048074406e-1_f64 * t25411 * t99389;
    let t99494 = -t99472 + t99475 + 0.51405703062096148812e-1_f64 * t93369 + 0.45699670022203476294e-2_f64 * t93372 + 0.25702851531048074406e-1_f64 * t93375 - t99480 - 0.96373646535613327357e-2_f64 * t99481 - 0.68540937416128198418e-2_f64 * t93378 - t99485 - t99487 - 0.13009920719177044025e-2_f64 * t93382 - 0.19274729307122665471e-1_f64 * t93384 - t99491 + t99493;
    t99494
}
