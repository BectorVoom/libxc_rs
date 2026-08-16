//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 220/1049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk220(t404: f64, t415: f64, t61: f64, t225: f64, t492: f64, t496: f64, t68: f64, t1011: f64, t1209: f64, t1206: f64, t357: f64, t475: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t1229 = 1.0_f64 / t415 / t404;
    let t1230 = t61 * t1229;
    let t1238 = t492 * t225;
    let t1239 = t496 * t496;
    let t1240 = 1.0_f64 / t1239;
    let t1241 = t68 * t1240;
    let t1243 = t1011 * t1209;
    let t1244 = t1206 * t1243;
    let t1246 = t357 * t475;
    (t1229, t1230, t1238, t1239, t1241, t1243, t1244, t1246)
}
