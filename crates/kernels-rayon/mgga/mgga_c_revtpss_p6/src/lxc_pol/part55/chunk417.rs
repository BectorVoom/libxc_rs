//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 417/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk417(t30: f64, t33: f64, t265: f64, t393: f64, t502: f64, t2071: f64, t207: f64, t2070: f64, t198: f64, t892: f64, t1940: f64, t45: f64, t57: f64, dens_threshold: f64, rho0: f64, rho1: f64, zeta_threshold: f64) -> (f64, f64, f64, f64) {
    let t31 = t30 <= zeta_threshold;
    let t34 = t33 <= zeta_threshold;
    let t120 = rho0 <= dens_threshold || t31;
    let t394 = t265 < t393;
    let t400 = rho1 <= dens_threshold || t34;
    let t503 = t265 < t502;
    let t2072 = t2071 * t30;
    let t2075 = t207 * t2070;
    let t2077 = t198 * t2075 * t892;
    let t2078 = piecewise3(t394, 0.0_f64, t2077);
    let t2081 = piecewise3(t120, t1940 * t2072 / 2.0_f64, t2078 * t45 / 2.0_f64);
    let t2082 = t2071 * t33;
    let t2085 = piecewise3(t503, 0.0_f64, t2077);
    let t2088 = piecewise3(t400, t1940 * t2082 / 2.0_f64, t2085 * t57 / 2.0_f64);
    let t2089 = t2081 + t2088;
    (t2075, t2078, t2085, t2089)
}
