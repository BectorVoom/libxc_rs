//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 632/1333 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk632(t33: f64, t265: f64, t502: f64, t2838: f64, t3804: f64, t1113: f64, t1304: f64, t2258: f64, t3351: f64, t504: f64, t57: f64, t606: f64, t895: f64, dens_threshold: f64, rho1: f64, zeta_threshold: f64) -> (f64, f64) {
    let t34 = t33 <= zeta_threshold;
    let t400 = rho1 <= dens_threshold || t34;
    let t503 = t265 < t502;
    let t3805 = piecewise3(t503, t3804, t2838);
    let t3812 = piecewise3(t400, t2838 * t33 / 2.0_f64 + t895 * t1113 + t265 * t3351 / 2.0_f64, t3805 * t57 / 2.0_f64 - t1304 * t606 - t504 * t2258 / 2.0_f64);
    (t3805, t3812)
}
