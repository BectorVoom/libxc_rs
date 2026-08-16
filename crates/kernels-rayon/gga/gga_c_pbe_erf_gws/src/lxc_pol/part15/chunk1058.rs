//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1058/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1058(t2319: f64, t3295: f64, t1123: f64, t6303: f64, t2255: f64, t1105: f64, t904: f64, t2258: f64, t1153: f64, t9521: f64, t8827: f64, t3223: f64, param_a_c: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t9601 = 7.0_f64 / 1152.0_f64 * t2319 * t3295;
    let t9603 = t1123 * t6303;
    let t9604 = t2255 * t9603;
    let t9607 = t1105 * param_a_c;
    let t9608 = t904 * t9607;
    let t9609 = t9608 * t2258;
    let t9612 = t1153 * t9521;
    let t9615 = t904 * t8827;
    let t9616 = t9615 * t3223;
    (t9601, t9603, t9604, t9607, t9609, t9612, t9616)
}
