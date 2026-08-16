//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 761/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk761(t30: f64, t259: f64, t379: f64, t3735: f64, t4027: f64, t1288: f64, t1289: f64, t1402: f64, t1490: f64, t3431: f64, t3743: f64, t381: f64, t45: f64, t580: f64, t581: f64, t826: f64, t999: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64) -> (f64, f64) {
    let t31 = t30 <= zeta_threshold;
    let t120 = rho0 <= dens_threshold || t31;
    let t380 = t259 < t379;
    let t4028 = piecewise3(t380, t4027, t3735);
    let t4035 = piecewise3(t120, t3735 * t30 / 2.0_f64 + t1402 * t580 / 2.0_f64 + t826 * t1288 / 2.0_f64 + t3743, t999 * t1289 / 2.0_f64 + t1490 * t581 / 2.0_f64 + t381 * t3431 / 2.0_f64 + t4028 * t45 / 2.0_f64);
    (t4028, t4035)
}
