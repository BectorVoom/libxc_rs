//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1166/1226 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1166(t23219: f64, t6547: f64, t23265: f64, t23030: f64, t23208: f64, t23168: f64, t23223: f64, t1882: f64, t81686: f64, t9537: f64, t1880: f64, t23218: f64, t23237: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t82143 = t6547 * t23219;
    let t82145 = t6547 * t23265;
    let t82147 = t23030 * t23208;
    let t82150 = t23168 * t23223;
    let t82153 = t81686 * t9537 * t1882;
    let t82156 = t1880 * t23237 * t23218;
    (t82143, t82145, t82147, t82150, t82153, t82156)
}
