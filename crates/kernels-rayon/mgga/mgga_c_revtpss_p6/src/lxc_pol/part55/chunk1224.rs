//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1224/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1224(t30: f64, t265: f64, t393: f64, t128014: f64, t128060: f64, t127592: f64, t127912: f64, t127939: f64, t127976: f64, t1469: f64, t32535: f64, t34127: f64, t4186: f64, t45: f64, t606: f64, t8671: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64) -> (f64, f64) {
    let t31 = t30 <= zeta_threshold;
    let t120 = rho0 <= dens_threshold || t31;
    let t394 = t265 < t393;
    let t128061 = t128014 + t128060;
    let t128062 = piecewise3(t394, 0.0_f64, t128061);
    let t128069 = piecewise3(t120, t127592 + t127912 + t127939 + t127976, t128062 * t45 / 2.0_f64 + t32535 * t1469 / 2.0_f64 + t34127 * t606 / 2.0_f64 + t8671 * t4186 / 2.0_f64);
    (t128061, t128069)
}
