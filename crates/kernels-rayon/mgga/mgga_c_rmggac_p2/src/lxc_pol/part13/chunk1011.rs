//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 1011/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk1011(t34884: f64, t9123: f64, t1240: f64, t1971: f64, t511: f64, t558: f64, t7230: f64, t4601: f64, t9008: f64, t27036: f64, t681: f64, t26346: f64, t7710: f64) -> (f64, f64, f64, f64, f64) {
    let t42144 = t34884 * t9123;
    let t42149 = t7230 * t1971 * t511 * t558 * t1240;
    let t42151 = t4601 * t9008;
    let t42156 = t27036 * t681;
    let t42159 = t26346 * t7710;
    (t42144, t42149, t42151, t42156, t42159)
}
