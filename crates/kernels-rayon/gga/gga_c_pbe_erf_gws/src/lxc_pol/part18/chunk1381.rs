//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1381/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1381(t3909: f64, t3955: f64, t13796: f64, t13859: f64, t3896: f64, t875: f64, t1118: f64, t3166: f64, t3854: f64, t898: f64, t13798: f64, t3989: f64) -> (f64, f64, f64, f64) {
    let t57707 = t3955 * t3909;
    let t57711 = t13859 * t13796 * t3896 * t875;
    let t57719 = t13859 * t13796 * t1118 * t3166;
    let t57728 = t898 * t3854;
    let t57731 = t3989 * t13796 * t57728 * t13798;
    (t57707, t57711, t57719, t57731)
}
