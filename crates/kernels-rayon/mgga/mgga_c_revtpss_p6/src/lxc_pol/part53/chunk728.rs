//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 728/1244 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk728(t33: f64, t265: f64, t502: f64, t2155: f64, t3801: f64, t1298: f64, t1300: f64, t198: f64, t336: f64, t5023: f64, t7193: f64, t7669: f64, t2159: f64, t57: f64, t606: f64, t7214: f64, dens_threshold: f64, rho1: f64, zeta_threshold: f64) -> (f64, f64, f64) {
    let t34 = t33 <= zeta_threshold;
    let t400 = rho1 <= dens_threshold || t34;
    let t503 = t265 < t502;
    let t7673 = t2155 * t3801;
    let t7677 = piecewise3(t503, t1300 * t198 * t336 * t7669 - t1298 * t5023 * t7673, t7193);
    let t7682 = piecewise3(t400, t7214, -t2159 * t606 / 2.0_f64 + t7677 * t57 / 2.0_f64);
    (t7673, t7677, t7682)
}
