//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 1745/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1745(t23228: f64, t6554: f64, t23171: f64, t23168: f64, t6556: f64, t6547: f64, t6573: f64, t214: f64, t852: f64) -> (f64, f64, f64, f64, f64) {
    let t23229 = t23228 * t6554;
    let t23230 = t23171 * t23229;
    let t23231 = 0.82246703342411321824e-2_f64 * t23230;
    let t23232 = t23168 * t6556;
    let t23235 = t6547 * t6573;
    let t23237 = t214 * t852;
    (t23229, t23231, t23232, t23235, t23237)
}
