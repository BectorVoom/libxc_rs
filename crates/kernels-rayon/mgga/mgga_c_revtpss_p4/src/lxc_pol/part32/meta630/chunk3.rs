//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 2034/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2034(t106494: f64, t26425: f64, t102888: f64, t106490: f64, t106498: f64, t106502: f64, t106520: f64, t106528: f64, t106572: f64, t106583: f64, t106602: f64, t1940: f64, t2403: f64, t27166: f64, t27376: f64, t27387: f64, t27391: f64, t27395: f64, t28291: f64, t28460: f64, t30420: f64, t605: f64, t7432: f64, t8020: f64) -> (f64, f64) {
    let t110717 = 6.0_f64 * t26425 * t106494;
    let t110745 = 6.0_f64 * t26425 * t106502 - 3.0_f64 * t26425 * t106490 + t110717 + 3.0_f64 * t28291 * t106498 - 3.0_f64 * t102888 * t27376 - t1940 * t28460 * t27387 - 3.0_f64 * t26425 * t106520 - t1940 * t28460 * t27391 - 3.0_f64 * t102888 * t27166 - t1940 * t7432 * t106583 + 3.0_f64 * t2403 * t8020 * t27395 + 6.0_f64 * t28291 * t106572 + t1940 * t30420 * t605 / 2.0_f64 - t1940 * t7432 * t106602 / 2.0_f64 - 3.0_f64 * t26425 * t106528;
    (t110717, t110745)
}
