//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1153/1333 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1153(t5: f64, t30: f64, t265: f64, t393: f64, t26798: f64, t117: f64, t2126: f64, t2327: f64, t25743: f64, t2129: f64, t2258: f64, t25459: f64, t45: f64, t606: f64, t7594: f64, t2138: f64, t3650: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t7 = piecewise3(0.0_f64 < t5, t5, -t5);
    let t8 = -t7 <= -0.999999999999e0_f64;
    let t31 = t30 <= zeta_threshold;
    let t120 = rho0 <= dens_threshold || t31;
    let t394 = t265 < t393;
    let t26799 = piecewise3(t8, 0.0_f64, t26798);
    let t26800 = t26799 * t117;
    let t26804 = t2126 * t2327;
    let t26809 = piecewise3(t394, 0.0_f64, t25743);
    let t26816 = piecewise3(t120, t25459, t26809 * t45 / 2.0_f64 + t7594 * t606 + t2129 * t2258 / 2.0_f64);
    let t26817 = t3650 * t2138;
    (t26799, t26800, t26804, t26809, t26816, t26817)
}
