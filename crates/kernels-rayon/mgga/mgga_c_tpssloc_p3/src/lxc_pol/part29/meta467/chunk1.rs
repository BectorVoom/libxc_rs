//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 1796/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1796(t23228: f64, t6554: f64, t23171: f64, t23168: f64, t6556: f64, t22975: f64, t22979: f64, t23191: f64, t23198: f64, t23202: f64, t23207: f64, t23209: f64, t23211: f64, t23215: f64, t23220: f64, t23224: f64, t23226: f64, t259: f64, t2597: f64, t2713: f64, t6632: f64, t6663: f64, t855: f64) -> (f64, f64, f64, f64) {
    let t23229 = t23228 * t6554;
    let t23230 = t23171 * t23229;
    let t23231 = 0.82246703342411321824e-2_f64 * t23230;
    let t23232 = t23168 * t6556;
    let t23233 = 0.76763589786250567036e-1_f64 * t23232;
    let t23234 = 4.0_f64 * t2713 * t6632 + 2.0_f64 * t855 * t22975 + 4.0_f64 * t855 * t22979 - t855 * t23191 - 2.0_f64 * t2713 * t6663 + 0.16449340668482264365e-1_f64 * t23198 + 4.0_f64 * t2597 * t6632 + t23202 * t259 + t23207 + 0.82246703342411321824e-2_f64 * t23209 + 2.0_f64 * t23211 * t259 - 6.0_f64 * t855 * t23215 - 0.82246703342411321825e-2_f64 * t23220 - 0.16449340668482264365e-1_f64 * t23224 + t23226 * t259 - t23231 + t23233;
    (t23229, t23231, t23232, t23234)
}
