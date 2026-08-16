//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 840/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk840(t6134: f64, t935: f64, t1398: f64, t30: f64, t1364: f64, t33: f64, t1338: f64, t93: f64) -> (f64, f64, f64, f64, f64) {
    let t6135 = t6134 * t935;
    let t6153 = t30 * t1398;
    let t6207 = t33 * t1364;
    let t6214 = t33 * t1398;
    let t6234 = t93 * t1338;
    (t6135, t6153, t6207, t6214, t6234)
}
