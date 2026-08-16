//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 348/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk348(t33: f64, t259: f64, t479: f64, t1021: f64, t1046: f64, t1086: f64, t1088: f64, t1093: f64, t1151: f64, t1153: f64, t198: f64, t330: f64, t826: f64, t1006: f64, t481: f64, t57: f64, t581: f64, dens_threshold: f64, rho1: f64, zeta_threshold: f64) -> (f64, f64) {
    let t34 = t33 <= zeta_threshold;
    let t386 = rho1 <= dens_threshold || t34;
    let t480 = t259 < t479;
    let t1157 = piecewise3(t480, t1151 * t1153 * t198 * t330 - t1021 + t1046 + t1086 + t1088 - t1093, t826);
    let t1162 = piecewise3(t386, t259 * t1006 / 2.0_f64 + t826 * t33 / 2.0_f64, t1157 * t57 / 2.0_f64 - t481 * t581 / 2.0_f64);
    (t1157, t1162)
}
