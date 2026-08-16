//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 422/1333 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk422(t30: f64, t265: f64, t393: f64, t1995: f64, t1966: f64, t45: f64, t343: f64, t55: f64, t136: f64, t473: f64, t479: f64, dens_threshold: f64, rho0: f64, sigma2: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t31 = t30 <= zeta_threshold;
    let t120 = rho0 <= dens_threshold || t31;
    let t394 = t265 < t393;
    let t2129 = piecewise3(t394, 0.0_f64, t1995);
    let t2132 = piecewise3(t120, t1966, t2129 * t45 / 2.0_f64);
    let t2133 = t55 * t343;
    let t2134 = t2133 * t136;
    let t2137 = t473 * sigma2;
    let t2138 = t2137 * t479;
    (t2129, t2132, t2133, t2134, t2137, t2138)
}
