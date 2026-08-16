//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 435/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk435(t202: f64, t631: f64, t184: f64, t582: f64, t611: f64, t185: f64, t1687: f64, t174: f64, t177: f64, t332: f64, t395: f64, t574: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t1729 = t202 * t631;
    let t1730 = t1729 * t184;
    let t1740 = t582 * t611;
    let t1741 = t185 * t1740;
    let t1743 = 0.25188888888888888889e-2_f64 * t1687;
    let t1754 = t174 * t332 * t177;
    let t1755 = 0.25188888888888888889e-2_f64 * t1754;
    let t1756 = t395 * t574;
    (t1729, t1730, t1740, t1741, t1743, t1754, t1755, t1756)
}
