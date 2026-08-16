//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 948/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk948(t23204: f64, t6555: f64, t23164: f64, t6572: f64, t6562: f64, t212: f64, t252: f64, t6554: f64, t23171: f64, t23168: f64, t6556: f64, t6547: f64, t6573: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t23205 = t23204 * t6555;
    let t23206 = t23164 * t23205;
    let t23207 = 0.16449340668482264365e-1_f64 * t23206;
    let t23208 = t23204 * t6572;
    let t23209 = t6562 * t23208;
    let t23228 = t212 * t252;
    let t23229 = t23228 * t6554;
    let t23230 = t23171 * t23229;
    let t23231 = 0.82246703342411321824e-2_f64 * t23230;
    let t23232 = t23168 * t6556;
    let t23233 = 0.76763589786250567036e-1_f64 * t23232;
    let t23235 = t6547 * t6573;
    (t23206, t23207, t23209, t23228, t23230, t23231, t23232, t23233, t23235)
}
