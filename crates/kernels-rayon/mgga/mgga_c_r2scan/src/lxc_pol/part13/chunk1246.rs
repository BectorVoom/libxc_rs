//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1246/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1246(t1035: f64, t1339: f64, t352: f64, t1343: f64, t3675: f64, t11148: f64, t11157: f64, t11162: f64, t11993: f64, t31912: f64, t31929: f64, t3420: f64, t37204: f64, t37209: f64, t37223: f64, t37226: f64) -> f64 {
    let t41058 = t1035 * t1339 * t352;
    let t41065 = t3675 * t1343;
    let t41086 = -0.1575e1_f64 * t3420 * t31929 - 0.354375e1_f64 * t37209 * t41058 - 0.126e2_f64 * t11157 * t11993 - 0.126e2_f64 * t3420 * t31912 - 0.63e1_f64 * t3420 * t41065 - 0.252e2_f64 * t11148 * t41058 - 0.567e2_f64 * t11162 * t41058 - 0.189e2_f64 * t37223 * t11993 - 0.945e1_f64 * t11148 * t41065 - 0.189e2_f64 * t11148 * t31912 - 0.2835e2_f64 * t37226 * t41058 - 0.4725e1_f64 * t37204 * t11993 - 0.4725e1_f64 * t11162 * t31912 - 0.23625e1_f64 * t11162 * t41065;
    t41086
}
