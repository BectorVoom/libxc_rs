//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 160/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk160(t30: f64, t33: f64, t489: f64, t498: f64, t187: f64, t497: f64, t490: f64, t199: f64, t493: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64) {
    let t31 = t30 <= zeta_threshold;
    let t34 = t33 <= zeta_threshold;
    let t499 = t489 * t498;
    let t501 = 0.19751673498613801407e-1_f64 * t497 * t187;
    let t502 = t490 * t490;
    let t503 = piecewise3(t31, t199, t502);
    let t504 = t493 * t493;
    let t505 = piecewise3(t34, t199, t504);
    let t507 = t503 / 2.0_f64 + t505 / 2.0_f64;
    (t499, t501, t502, t504, t507)
}
