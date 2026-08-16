//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1362/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1362(t14657: f64, t8695: f64, t8790: f64, t13776: f64, t28657: f64, t3808: f64, t3975: f64, t44201: f64, t1113: f64, t13781: f64, t3747: f64, t3972: f64, t938: f64) -> (f64, f64, f64, f64, f64) {
    let t57402 = t14657 * t8695;
    let t57404 = t14657 * t8790;
    let t57410 = t13776 * t3975 * t3808 * t28657;
    let t57415 = t13776 * t3975 * t44201;
    let t57422 = t3972 * t13781 * t1113 * t3747 * t938;
    (t57402, t57404, t57410, t57415, t57422)
}
