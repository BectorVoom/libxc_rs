//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 636/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk636(t4972: f64, t571: f64, t11: f64, t4360: f64, t572: f64, t4940: f64, t4941: f64, t4943: f64, t4945: f64, t4947: f64, t4954: f64, t4960: f64, t4965: f64, t4969: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t4973 = t571 * t4972;
    let t4974 = t11 * t4973;
    let t4976 = t572 * t4360;
    let t4977 = t571 * t4976;
    let t4978 = t11 * t4977;
    let t4980 = t4940 + 0.25188888888888888889e-2_f64 * t4941 - 0.12594444444444444445e-2_f64 * t4943 + 0.37783333333333333335e-2_f64 * t4945 - 0.18891666666666666667e-2_f64 * t4947 + 0.20990740740740740742e-2_f64 * t4954 - 0.75566666666666666669e-2_f64 * t4960 + 0.37783333333333333335e-2_f64 * t4965 + 0.11335e-1_f64 * t4969 - 0.11335e-1_f64 * t4974 + 0.18891666666666666667e-2_f64 * t4978;
    (t4973, t4974, t4976, t4977, t4978, t4980)
}
