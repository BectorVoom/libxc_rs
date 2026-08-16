//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1104/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1104(t14091: f64, t14093: f64, t3139: f64, t6409: f64, t4028: f64, t331: f64, t911: f64, t56: f64, t863: f64) -> (f64, f64, f64, f64, f64) {
    let t14094 = t14091 * t14093;
    let t14096 = t3139 * t6409;
    let t14097 = t4028 * t14096;
    let t14099 = t911 * t331;
    let t14101 = t863 * t14099 * t56;
    (t14094, t14096, t14097, t14099, t14101)
}
