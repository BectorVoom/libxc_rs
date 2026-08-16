//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 366/1384 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk366(t25: f64, t28: f64, t1294: f64, t763: f64, t532: f64, t571: f64, t514: f64, t606: f64, t517: f64, t1081: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64) {
    let t26 = t25 <= zeta_threshold;
    let t29 = t28 <= zeta_threshold;
    let t1296 = 0.5848223622634646207e0_f64 * t1294 * t763;
    let t1297 = t532 * t571;
    let t1298 = 1.0_f64 / t514;
    let t1301 = piecewise3(t26, 0.0_f64, 2.0_f64 / 3.0_f64 * t1298 * t606);
    let t1302 = 1.0_f64 / t517;
    let t1305 = piecewise3(t29, 0.0_f64, 2.0_f64 / 3.0_f64 * t1302 * t1081);
    let t1307 = t1301 / 2.0_f64 + t1305 / 2.0_f64;
    (t1296, t1297, t1298, t1302, t1307)
}
