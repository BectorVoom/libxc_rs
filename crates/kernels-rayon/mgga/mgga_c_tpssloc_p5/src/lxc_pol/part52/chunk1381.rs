//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1381/1400 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1381(t1874: f64, t96238: f64, t27863: f64, t6535: f64, t120045: f64, t120047: f64, t120049: f64, t120051: f64, t120053: f64, t120055: f64, t120057: f64, t120063: f64, t31055: f64, t31057: f64, t31060: f64) -> f64 {
    let t123155 = t96238 * t1874;
    let t123164 = t27863 * t6535;
    let t123166 = -2.0_f64 * t123155 - 2.0_f64 * t120045 - 2.0_f64 * t120047 - 2.0_f64 * t120049 - 2.0_f64 * t120051 - 2.0_f64 * t120053 - 2.0_f64 * t120055 - 2.0_f64 * t120057 - t31055 - t31057 - t31060 - 2.0_f64 * t123164 - t120063;
    t123166
}
