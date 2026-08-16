//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1193/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1193(t5: f64, t30: f64, t265: f64, t393: f64, t30714: f64, t117: f64, t2126: f64, t5883: f64, t29930: f64, t1469: f64, t2129: f64, t29726: f64, t45: f64, t5825: f64, t8161: f64, t2142: f64, t6587: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t7 = piecewise3(0.0_f64 < t5, t5, -t5);
    let t8 = -t7 <= -0.999999999999e0_f64;
    let t31 = t30 <= zeta_threshold;
    let t120 = rho0 <= dens_threshold || t31;
    let t394 = t265 < t393;
    let t30715 = piecewise3(t8, 0.0_f64, t30714);
    let t30716 = t30715 * t117;
    let t30724 = t2126 * t5883;
    let t30727 = piecewise3(t394, 0.0_f64, t29930);
    let t30734 = piecewise3(t120, t29726, t30727 * t45 / 2.0_f64 + t8161 * t1469 + t2129 * t5825 / 2.0_f64);
    let t30735 = t2142 * t6587;
    (t30715, t30716, t30724, t30727, t30734, t30735)
}
