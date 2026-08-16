//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1066/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1066(t6617: f64, t2142: f64, t3805: f64, t2323: f64, t3871: f64, t9144: f64, t3131: f64, t3139: f64, t3166: f64, t2168: f64, t3912: f64, t6335: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t11974 = 35.0_f64 / 432.0_f64 * t6617;
    let t11975 = t3805 * t2142;
    let t11976 = 7.0_f64 / 288.0_f64 * t11975;
    let t11977 = t2323 * t3871;
    let t11979 = 35.0_f64 / 216.0_f64 * t9144;
    let t11981 = t3139 * t3131 * t3166;
    let t11983 = t2168 * t11981 / 48.0_f64;
    let t11984 = t3912 * t6335;
    (t11974, t11976, t11977, t11979, t11981, t11983, t11984)
}
