//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3349/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3349(t2857: f64, t60717: f64, t141: f64, t930: f64, t41361: f64, t41363: f64, t41610: f64, t51967: f64, t51973: f64, t51978: f64, t63299: f64, t63304: f64, t63308: f64, t63311: f64, t63315: f64, t63320: f64, t63325: f64, t63328: f64) -> (f64, f64, f64) {
    let t63330 = t2857 * t60717;
    let t63332 = t141 * t930 * t63330;
    let t63334 = 0.59793333333333333334e0_f64 * t63299 + 0.39862222222222222223e1_f64 * t63304 - 0.71752000000000000002e1_f64 * t63308 + t41610 - 0.98587999999999999998e0_f64 * t63311 + 0.197176e1_f64 * t63315 + 0.19931111111111111111e0_f64 * t51967 - 0.5314962962962962963e0_f64 * t51973 + 0.62007901234567901235e0_f64 * t51978 + 0.10954222222222222222e0_f64 * t63320 + 0.62007901234567901237e0_f64 * t41361 + 0.26574814814814814816e0_f64 * t41363 - 0.13287407407407407407e1_f64 * t63325 + 0.47834666666666666668e1_f64 * t63328 + 0.32862666666666666666e0_f64 * t63332;
    (t63330, t63332, t63334)
}
