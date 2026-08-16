//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1216/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1216(t1894: f64, t7489: f64, t1898: f64, t2743: f64, t1902: f64, t2746: f64, t5797: f64, t713: f64, t7510: f64, t694: f64, t7518: f64, t5771: f64, t7312: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t21220 = 3.0_f64 * t7489 * t1894;
    let t21221 = t2743 * t1898;
    let t21223 = 0.48245938496077605201e2_f64 * t21221 * t1902;
    let t21225 = 1.0_f64 * t2746 * t5797;
    let t21226 = t7510 * t713;
    let t21229 = t7518 * t694;
    let t21233 = 18.0_f64 * t5771 * t7312;
    (t21220, t21223, t21225, t21226, t21229, t21233)
}
