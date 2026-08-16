//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 872/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk872(t1828: f64, t7495: f64, t5218: f64, t5212: f64, t626: f64, t661: f64, t954: f64, t617: f64, t5211: f64, t1697: f64, t422: f64, t7115: f64) -> (f64, f64, f64, f64) {
    let t7496 = t7495 * t1828;
    let t7498 = 16.0_f64 / 45.0_f64 * t5218 * t7496;
    let t7499 = t5212 * t626;
    let t7500 = t954 * t661;
    let t7502 = t7499 * t7500 * t617;
    let t7504 = 16.0_f64 / 45.0_f64 * t5211 * t7502;
    let t7505 = t5212 * t1697;
    let t7506 = t7500 * t422;
    let t7507 = t7505 * t7506;
    let t7509 = 16.0_f64 / 45.0_f64 * t7115 * t7507;
    (t7498, t7504, t7506, t7509)
}
