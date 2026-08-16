//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 449/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk449(t1689: f64, t1694: f64, t1700: f64, t1704: f64, t1743: f64, t203: f64, t184: f64, t221: f64, t174: f64, t177: f64, t332: f64, t395: f64, t574: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t1748 = -t1743 - 0.12594444444444444445e-2_f64 * t1689 + 0.12594444444444444445e-2_f64 * t1694 - 0.37783333333333333334e-2_f64 * t1700 + 0.18891666666666666667e-2_f64 * t1704;
    let t1749 = t203 * t1748;
    let t1750 = t1749 * t184;
    let t1752 = 2.0_f64 / 15.0_f64 * t1750 * t221;
    let t1754 = t174 * t332 * t177;
    let t1755 = 0.25188888888888888889e-2_f64 * t1754;
    let t1756 = t395 * t574;
    (t1748, t1749, t1750, t1752, t1754, t1755, t1756)
}
