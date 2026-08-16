//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2136/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2136(t29598: f64, t775: f64, t25207: f64, t1940: f64, t2255: f64, t7783: f64, t77425: f64, t106498: f64, t106502: f64, t106510: f64, t106516: f64, t106520: f64, t106528: f64, t1468: f64, t2403: f64, t25206: f64, t27158: f64, t27166: f64, t27173: f64, t27364: f64, t27368: f64, t27391: f64, t29705: f64, t605: f64, t7091: f64, t7092: f64, t7787: f64, t98637: f64, t99555: f64) -> (f64, f64, f64) {
    let t106533 = t29598 * t775;
    let t106534 = t25207 * t106533;
    let t106539 = 2.0_f64 * t1940 * t7783 * t2255;
    let t106540 = t25207 * t77425;
    let t106543 = 3.0_f64 * t27158 * t106498 + 6.0_f64 * t25206 * t106502 + 3.0_f64 * t2403 * t7783 * t27173 + t1940 * t27364 * t1468 - t1940 * t7091 * t106510 / 2.0_f64 - t1940 * t99555 * t7787 - t1940 * t106516 * t7092 / 2.0_f64 - 3.0_f64 * t25206 * t106520 - 3.0_f64 * t98637 * t27166 + t1940 * t29705 * t605 / 2.0_f64 - 3.0_f64 * t25206 * t106528 - t1940 * t27368 * t27391 - 6.0_f64 * t27158 * t106534 + t106539 - 3.0_f64 / 2.0_f64 * t25206 * t106540;
    (t106533, t106539, t106543)
}
