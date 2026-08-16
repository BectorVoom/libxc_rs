//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 1177/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk1177(t30: f64, t265: f64, t393: f64, t127143: f64, t127180: f64, t127112: f64, t126434: f64, t1469: f64, t32059: f64, t33867: f64, t4186: f64, t45: f64, t606: f64, t8543: f64, t27375: f64, t27799: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64) -> (f64, f64, f64) {
    let t31 = t30 <= zeta_threshold;
    let t120 = rho0 <= dens_threshold || t31;
    let t394 = t265 < t393;
    let t127181 = t127143 + t127180;
    let t127182 = piecewise3(t394, t127112, t127181);
    let t127189 = piecewise3(t120, t126434, t127182 * t45 / 2.0_f64 + t32059 * t1469 / 2.0_f64 + t33867 * t606 / 2.0_f64 + t8543 * t4186 / 2.0_f64);
    let t127190 = t27799 * t27375;
    (t127181, t127189, t127190)
}
