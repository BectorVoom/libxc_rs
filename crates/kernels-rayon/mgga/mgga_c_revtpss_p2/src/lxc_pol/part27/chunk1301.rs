//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1301/1333 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1301(t30: f64, t265: f64, t393: f64, t94213: f64, t10326: f64, t2129: f64, t2258: f64, t26809: f64, t45: f64, t606: f64, t7594: f64, t93409: f64, t12627: f64, t2142: f64, t12640: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64) -> (f64, f64, f64) {
    let t31 = t30 <= zeta_threshold;
    let t120 = rho0 <= dens_threshold || t31;
    let t394 = t265 < t393;
    let t96848 = piecewise3(t394, 0.0_f64, t94213);
    let t96858 = piecewise3(t120, t93409, t96848 * t45 / 2.0_f64 + 3.0_f64 / 2.0_f64 * t26809 * t606 + 3.0_f64 / 2.0_f64 * t7594 * t2258 + t2129 * t10326 / 2.0_f64);
    let t96861 = t12627 * t2142;
    let t96866 = t12640 * t2142;
    (t96858, t96861, t96866)
}
