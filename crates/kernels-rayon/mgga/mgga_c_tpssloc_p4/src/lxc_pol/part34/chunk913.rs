//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 913/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk913(t21429: f64, t21479: f64, t225: f64, t68: f64, t369: f64, t14211: f64, t17712: f64, t4582: f64, t21126: f64, t977: f64, t21122: f64, t2979: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t21480 = t21429 + t21479;
    let t21481 = t21480 * t225;
    let t21482 = t21481 * t68;
    let t21483 = t21482 * t369;
    let t21486 = t17712 * t14211;
    let t21487 = t4582 * t21486;
    let t21490 = t977 * t21126;
    let t21493 = t2979 * t21122;
    (t21480, t21481, t21483, t21487, t21490, t21493)
}
