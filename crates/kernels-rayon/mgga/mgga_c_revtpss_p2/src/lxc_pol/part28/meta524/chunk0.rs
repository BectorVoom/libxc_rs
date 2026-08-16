//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 1950/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1950(t33: f64, t265: f64, t502: f64, t27754: f64, t1469: f64, t2003: f64, t27821: f64, t4186: f64, t57: f64, t606: f64, t7215: f64, t7877: f64, t27762: f64, t196: f64, t197: f64, t5528: f64, dens_threshold: f64, rho1: f64, zeta_threshold: f64) -> (f64, f64, f64) {
    let t34 = t33 <= zeta_threshold;
    let t400 = rho1 <= dens_threshold || t34;
    let t503 = t265 < t502;
    let t27822 = piecewise3(t503, 0.0_f64, t27754);
    let t27829 = piecewise3(t400, t27821, -t7215 * t1469 / 2.0_f64 - t2003 * t4186 / 2.0_f64 + t27822 * t57 / 2.0_f64 - t7877 * t606 / 2.0_f64);
    let t27830 = t27762 + t27829;
    let t27833 = t5528 * t196 * t197;
    (t27822, t27830, t27833)
}
