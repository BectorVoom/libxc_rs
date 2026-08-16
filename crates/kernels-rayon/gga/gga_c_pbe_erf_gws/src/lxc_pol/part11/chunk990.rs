//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 990/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk990(t2106: f64, t3772: f64, t1: f64, t2057: f64, t2062: f64, t3701: f64, t10024: f64, t2096: f64, t11387: f64, t331: f64, t4395: f64, t3916: f64, t6154: f64) -> (f64, f64, f64, f64, f64) {
    let t35109 = t3772 * t2106;
    let t35128 = t3701 * t2057 * t1 * t2062;
    let t35137 = t10024 * t2096;
    let t35187 = t11387 * t331;
    let t35188 = t4395 * t35187;
    let t35277 = t3916 * t6154;
    (t35109, t35128, t35137, t35188, t35277)
}
