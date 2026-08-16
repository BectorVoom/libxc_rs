//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1313/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1313(t11758: f64, t4039: f64, t11531: f64, t14015: f64, t8991: f64, t9035: f64, t11754: f64, t2080: f64, t3107: f64, t12044: f64, t14092: f64, t38537: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t57017 = t4039 * t11758;
    let t57019 = t14015 * t11531;
    let t57021 = t9035 * t8991;
    let t57023 = t4039 * t11754;
    let t57026 = t2080 * t3107;
    let t57028 = t57026 * t14092 * t12044;
    let t57030 = t2080 * t38537;
    (t57017, t57019, t57021, t57023, t57028, t57030)
}
