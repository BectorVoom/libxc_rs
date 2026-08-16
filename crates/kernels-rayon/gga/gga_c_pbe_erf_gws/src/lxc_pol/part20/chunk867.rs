//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 867/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk867(t3128: f64, t6332: f64, t2494: f64, t5: f64, t337: f64, t2147: f64, t2153: f64, t838: f64, t863: f64, t3142: f64, t3172: f64, t6484: f64) -> (f64, f64, f64, f64, f64) {
    let t8960 = 7.0_f64 / 72.0_f64 * t3128 * t6332;
    let t8961 = t5 * t2494;
    let t8962 = t337 * t8961;
    let t8963 = t2147 * t8962;
    let t8967 = t863 * t2153 * t838;
    let t8969 = 7.0_f64 / 72.0_f64 * t8967 * t3142;
    let t8971 = 7.0_f64 / 144.0_f64 * t6484 * t3172;
    (t8960, t8962, t8963, t8969, t8971)
}
