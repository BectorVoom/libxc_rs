//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1292/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1292(t42046: f64, t42061: f64, t42077: f64, t42092: f64, t893: f64, t913: f64, t2840: f64, t275: f64, t2843: f64, t41995: f64, t10619: f64, t942: f64) -> (f64, f64, f64) {
    let t42097 = 1.0_f64 * t893 * (t42046 + t42061 + t42077 + t42092) * t913;
    let t42098 = t2840 * t2840;
    let t42100 = t275 / t42098;
    let t42101 = t2843 * t2843;
    let t42102 = 1.0_f64 / t42101;
    let t42105 = 0.24955700379505800916e5_f64 * t42100 * t41995 * t42102;
    let t42106 = t10619 * t942;
    (t42097, t42105, t42106)
}
