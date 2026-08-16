//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 597/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk597(t30: f64, t265: f64, t502: f64, t1966: f64, t1996: f64, t45: f64, t1963: f64, t33: f64, t1940: f64, t1995: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64) -> (f64, f64, f64) {
    let t31 = t30 <= zeta_threshold;
    let t120 = rho0 <= dens_threshold || t31;
    let t503 = t265 < t502;
    let t1999 = piecewise3(t120, t1966, t1996 * t45 / 2.0_f64);
    let t2000 = t1963 * t33;
    let t2002 = t1940 * t2000 / 2.0_f64;
    let t2003 = piecewise3(t503, 0.0_f64, t1995);
    (t1999, t2002, t2003)
}
