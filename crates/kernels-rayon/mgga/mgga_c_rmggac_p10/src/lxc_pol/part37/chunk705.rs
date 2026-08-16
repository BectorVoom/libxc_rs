//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 705/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk705(t201: f64, t14056: f64, t14371: f64, t13889: f64, t14368: f64, t13892: f64, t14007: f64, t26004: f64, t3051: f64, t3052: f64, t1343: f64, t69097: f64, t69101: f64, t71: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t69635 = t201 * t201;
    let t69648 = t14371 * t14056;
    let t69662 = t14368 * t13889;
    let t69663 = 0.16351352353374609375e-5_f64 * t69662;
    let t69664 = t14368 * t13892;
    let t69665 = 0.24527028530061914062e-5_f64 * t69664;
    let t69666 = t14368 * t14007;
    let t69667 = 0.24527028530061914062e-5_f64 * t69666;
    let t69670 = t3051 / t3052 / t26004;
    let t69674 = t69670 * t69097 * t1343 * t71 * t69101;
    (t69635, t69648, t69663, t69665, t69667, t69674)
}
