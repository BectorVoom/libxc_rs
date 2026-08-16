//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1097/1225 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1097(t33: f64, t265: f64, t502: f64, t26625: f64, t2085: f64, t2258: f64, t26665: f64, t57: f64, t606: f64, t7468: f64, t26633: f64, t2051: f64, t2327: f64, t2107: f64, t25177: f64, dens_threshold: f64, rho1: f64, zeta_threshold: f64) -> (f64, f64, f64, f64) {
    let t34 = t33 <= zeta_threshold;
    let t400 = rho1 <= dens_threshold || t34;
    let t503 = t265 < t502;
    let t26666 = piecewise3(t503, 0.0_f64, t26625);
    let t26673 = piecewise3(t400, t26665, t26666 * t57 / 2.0_f64 - t7468 * t606 - t2085 * t2258 / 2.0_f64);
    let t26674 = t26633 + t26673;
    let t26676 = t2051 * t2327;
    let t26679 = t2107 * t25177;
    (t26666, t26674, t26676, t26679)
}
