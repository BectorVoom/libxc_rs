//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 957/1244 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk957(t30: f64, t265: f64, t393: f64, t27754: f64, t1469: f64, t2129: f64, t27408: f64, t4186: f64, t45: f64, t606: f64, t7594: f64, t8161: f64, t5273: f64, t7617: f64, t5291: f64, t7616: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64) -> (f64, f64, f64) {
    let t31 = t30 <= zeta_threshold;
    let t120 = rho0 <= dens_threshold || t31;
    let t394 = t265 < t393;
    let t28998 = piecewise3(t394, 0.0_f64, t27754);
    let t29005 = piecewise3(t120, t27408, t7594 * t1469 / 2.0_f64 + t2129 * t4186 / 2.0_f64 + t28998 * t45 / 2.0_f64 + t8161 * t606 / 2.0_f64);
    let t29010 = t5273 * t7617;
    let t29019 = t7616 * t5291;
    (t29005, t29010, t29019)
}
