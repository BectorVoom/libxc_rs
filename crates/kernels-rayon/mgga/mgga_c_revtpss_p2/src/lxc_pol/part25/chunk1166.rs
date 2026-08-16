//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1166/1360 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1166(t33: f64, t265: f64, t502: f64, t25743: f64, t2003: f64, t2258: f64, t25791: f64, t57: f64, t606: f64, t7215: f64, t25751: f64, t4135: f64, t4147: f64, t2034: f64, t2014: f64, dens_threshold: f64, rho1: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64) {
    let t34 = t33 <= zeta_threshold;
    let t400 = rho1 <= dens_threshold || t34;
    let t503 = t265 < t502;
    let t25792 = piecewise3(t503, 0.0_f64, t25743);
    let t25799 = piecewise3(t400, t25791, t25792 * t57 / 2.0_f64 - t7215 * t606 - t2003 * t2258 / 2.0_f64);
    let t25800 = t25751 + t25799;
    let t25802 = t4147 * t4135;
    let t25803 = t2034 * t25802;
    let t25804 = t2014 * t25803;
    (t25792, t25800, t25802, t25803, t25804)
}
