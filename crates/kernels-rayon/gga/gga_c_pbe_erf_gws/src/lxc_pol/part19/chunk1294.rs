//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1294/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1294(t28652: f64, t3808: f64, t3972: f64, t3975: f64, t361: f64, t56296: f64, t13917: f64, t3223: f64, t13796: f64, t14423: f64, t3166: f64, t3989: f64) -> (f64, f64, f64) {
    let t56678 = t3972 * t3975 * t3808 * t28652;
    let t56684 = t361 * t56296;
    let t56686 = t13917 * t56684 * t3223;
    let t56697 = t3989 * t13796 * t14423 * t3166;
    (t56678, t56686, t56697)
}
