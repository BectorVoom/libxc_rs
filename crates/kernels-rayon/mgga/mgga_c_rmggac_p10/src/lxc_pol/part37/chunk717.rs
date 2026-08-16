//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 717/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk717(t14150: f64, t290: f64, t35253: f64, t70127: f64, t10570: f64, t14077: f64, t14154: f64, t12200: f64, t13801: f64, t388: f64, t669: f64, t7933: f64, t7934: f64) -> (f64, f64, f64, f64) {
    let t70130 = t70127 * t35253 * t290 * t14150;
    let t70131 = 0.15372131649401827112e-4_f64 * t70130;
    let t70149 = t10570 * t14077 * t14154;
    let t70156 = t12200 * t14077 * t13801;
    let t70169 = t7933 * t7934 * t388 * t669;
    (t70131, t70149, t70156, t70169)
}
