//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1355/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1355(t11737: f64, t14637: f64, t3974: f64, t3990: f64, t13796: f64, t3887: f64, t3989: f64, t875: f64, t376: f64, t3854: f64, t13859: f64, t2171: f64) -> (f64, f64, f64, f64) {
    let t57311 = t14637 * t3990 * t3974 * t11737;
    let t57319 = t3989 * t13796 * t3887 * t875;
    let t57321 = t376 * t3854;
    let t57324 = t13859 * t13796 * t57321 * t2171;
    (t57311, t57319, t57321, t57324)
}
