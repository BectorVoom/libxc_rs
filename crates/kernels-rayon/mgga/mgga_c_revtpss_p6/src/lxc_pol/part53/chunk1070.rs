//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1070/1244 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1070(t30: f64, t265: f64, t393: f64, t1936: f64, t8233: f64, t651: f64, t33866: f64, t1469: f64, t33748: f64, t45: f64, t8752: f64, t33902: f64, t196: f64, t197: f64, t8237: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64) {
    let t31 = t30 <= zeta_threshold;
    let t120 = rho0 <= dens_threshold || t31;
    let t394 = t265 < t393;
    let t34382 = t8233 * t1936;
    let t34383 = t651 * t34382;
    let t34388 = piecewise3(t394, 0.0_f64, t33866);
    let t34393 = piecewise3(t120, t33748, t8752 * t1469 / 2.0_f64 + t34388 * t45 / 2.0_f64);
    let t34394 = t34393 + t33902;
    let t34399 = t8237 * t196 * t197;
    (t34382, t34383, t34388, t34394, t34399)
}
