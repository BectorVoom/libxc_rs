//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1323/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1323(t14242: f64, t5559: f64, t1395: f64, t226: f64, t3664: f64, t19766: f64, t6134: f64, t45241: f64, t782: f64, t4716: f64, t44994: f64, t4764: f64, t818: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t70001 = t5559 * t14242;
    let t70030 = t1395 * t3664 * t226;
    let t70039 = t6134 * t19766;
    let t70042 = t45241 * t782;
    let t70046 = t4716 * t782;
    let t70060 = t44994 * t226;
    let t70063 = t4764 * t818;
    (t70001, t70030, t70039, t70042, t70046, t70060, t70063)
}
