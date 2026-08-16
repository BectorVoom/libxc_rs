//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 758/1203 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk758(t30: f64, t265: f64, t393: f64, t651: f64, t8749: f64, t8542: f64, t45: f64, t8498: f64, t1936: f64, t7586: f64, t196: f64, t2165: f64, t197: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t31 = t30 <= zeta_threshold;
    let t120 = rho0 <= dens_threshold || t31;
    let t394 = t265 < t393;
    let t8750 = t651 * t8749;
    let t8752 = piecewise3(t394, 0.0_f64, t8542);
    let t8755 = piecewise3(t120, t8498, t8752 * t45 / 2.0_f64);
    let t8758 = t7586 * t1936;
    let t8763 = t2165 * t196;
    let t8764 = t8763 * t197;
    (t8750, t8752, t8755, t8758, t8763, t8764)
}
