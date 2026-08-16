//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1306/1341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1306(t33: f64, t265: f64, t502: f64, t114149: f64, t114199: f64, t114089: f64, t1469: f64, t2003: f64, t22671: f64, t29978: f64, t57: f64, t5825: f64, t7877: f64, t2014: f64, t30111: f64, t5542: f64, dens_threshold: f64, rho1: f64, zeta_threshold: f64) -> (f64, f64) {
    let t34 = t33 <= zeta_threshold;
    let t400 = rho1 <= dens_threshold || t34;
    let t503 = t265 < t502;
    let t114200 = t114149 + t114199;
    let t114201 = piecewise3(t503, 0.0_f64, t114089);
    let t114211 = piecewise3(t400, t114200, t114201 * t57 / 2.0_f64 - 3.0_f64 / 2.0_f64 * t29978 * t1469 - 3.0_f64 / 2.0_f64 * t7877 * t5825 - t2003 * t22671 / 2.0_f64);
    let t114216 = 3.0_f64 * t2014 * t30111 * t5542;
    (t114211, t114216)
}
