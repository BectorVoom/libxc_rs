//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 717/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk717(t10570: f64, t14077: f64, t14154: f64, t12200: f64, t13801: f64, t388: f64, t669: f64, t7933: f64, t7934: f64, t3047: f64, t49: f64, t35688: f64, t7935: f64) -> (f64, f64, f64, f64, f64) {
    let t70149 = t10570 * t14077 * t14154;
    let t70156 = t12200 * t14077 * t13801;
    let t70169 = t7933 * t7934 * t388 * t669;
    let t70171 = t3047 * t49;
    let t70173 = t35688 * t70171 * t7935;
    (t70149, t70156, t70169, t70171, t70173)
}
