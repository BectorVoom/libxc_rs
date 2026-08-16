//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 702/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk702(t2500: f64, t68756: f64, t128: f64, t1330: f64, t793: f64, t14229: f64, t7254: f64, t7778: f64, t7879: f64, t903: f64, t641: f64, t7553: f64, t7555: f64) -> (f64, f64, f64, f64, f64) {
    let t69518 = t2500 * t68756;
    let t69521 = t793 * t128 * t1330;
    let t69568 = t7254 * t14229;
    let t69574 = t903 * t7778 * t7879;
    let t69583 = t7553 * t7555 * t641;
    (t69518, t69521, t69568, t69574, t69583)
}
