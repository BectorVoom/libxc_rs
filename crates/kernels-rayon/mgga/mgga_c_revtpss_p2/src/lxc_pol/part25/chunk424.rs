//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 424/1360 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk424(t33: f64, t265: f64, t502: f64, t1963: f64, t1940: f64, t1995: f64, t57: f64, t1999: f64, dens_threshold: f64, rho1: f64, zeta_threshold: f64) -> (f64, f64, f64) {
    let t34 = t33 <= zeta_threshold;
    let t400 = rho1 <= dens_threshold || t34;
    let t503 = t265 < t502;
    let t2000 = t1963 * t33;
    let t2002 = t1940 * t2000 / 2.0_f64;
    let t2003 = piecewise3(t503, 0.0_f64, t1995);
    let t2006 = piecewise3(t400, t2002, t2003 * t57 / 2.0_f64);
    let t2007 = t1999 + t2006;
    (t2000, t2003, t2007)
}
