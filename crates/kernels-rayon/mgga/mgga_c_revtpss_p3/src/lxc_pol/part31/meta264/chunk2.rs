//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 1181/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1181(t30: f64, t1996: f64, t45: f64, t606: f64, t7099: f64, t7194: f64, t33: f64, t775: f64, t890: f64, t1113: f64, t1940: f64, t1963: f64, t2403: f64, t7087: f64, t7091: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64) -> (f64, f64, f64, f64) {
    let t31 = t30 <= zeta_threshold;
    let t120 = rho0 <= dens_threshold || t31;
    let t7199 = piecewise3(t120, t7099, t1996 * t606 / 2.0_f64 + t7194 * t45 / 2.0_f64);
    let t7200 = t33 * t775;
    let t7207 = t33 * t890;
    let t7214 = 3.0_f64 / 2.0_f64 * t2403 * t1963 * t7200 + t1940 * t7087 * t33 / 2.0_f64 - t1940 * t7091 * t7207 / 2.0_f64 + t1940 * t1963 * t1113 / 2.0_f64;
    (t7199, t7200, t7207, t7214)
}
