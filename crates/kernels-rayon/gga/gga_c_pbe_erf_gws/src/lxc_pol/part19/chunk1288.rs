//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1288/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1288(t11797: f64, t13917: f64, t13919: f64, t1161: f64, t3258: f64, t53161: f64, t816: f64, t820: f64, t11559: f64, t53156: f64, t11530: f64, t50998: f64, t51066: f64) -> (f64, f64, f64, f64) {
    let t56534 = t13917 * t13919 * t11797;
    let t56545 = t13917 * t53161 * t3258 * t816 * t1161 * t820;
    let t56548 = t13917 * t53156 * t11559;
    let t56551 = t50998 * t51066 * t11530;
    (t56534, t56545, t56548, t56551)
}
