//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 361/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk361(t28: f64, t265: f64, t504: f64, t500: f64, t1096: f64, t1121: f64, t1161: f64, t1163: f64, t1168: f64, t1254: f64, t193: f64, t336: f64, t873: f64, t1081: f64, t506: f64, t52: f64, t607: f64, dens_threshold: f64, rho1: f64, zeta_threshold: f64) -> (f64, f64, f64) {
    let t29 = t28 <= zeta_threshold;
    let t401 = rho1 <= dens_threshold || t29;
    let t505 = t265 < t504;
    let t1256 = 1.0_f64 / t500;
    let t1260 = piecewise3(t505, t1254 * t1256 * t193 * t336 - t1096 + t1121 + t1161 + t1163 - t1168, t873);
    let t1265 = piecewise3(t401, t265 * t1081 / 2.0_f64 + t873 * t28 / 2.0_f64, t1260 * t52 / 2.0_f64 - t506 * t607 / 2.0_f64);
    (t1256, t1260, t1265)
}
