//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 929/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk929(t116: f64, t366: f64, t798: f64, t799: f64, t311: f64, t19: f64, t2331: f64, t301: f64, t305: f64, t2082: f64) -> (f64, f64, f64) {
    let t19525 = 0.6693920255418271605e1_f64 * t798 * t799 * t366 * t116;
    let t19530 = t311 * t311;
    let t19537 = 0.34072858057724757727e0_f64 * t305 / t19530 * t2331 * t301 * t19 * t799;
    let t19560 = t2082 * t2082;
    let t19561 = 1.0_f64 / t19560;
    (t19525, t19537, t19561)
}
