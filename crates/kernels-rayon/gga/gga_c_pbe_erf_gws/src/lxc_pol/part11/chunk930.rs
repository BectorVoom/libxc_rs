//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 930/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk930(t19561: f64, t2105: f64, t825: f64, t2365: f64, t4395: f64, t337: f64, t6658: f64, t2306: f64, t4422: f64, t56: f64, t2118: f64, t816: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t19562 = t19561 * t2105;
    let t19563 = t19562 * t825;
    let t19637 = t4395 * t2365;
    let t19693 = t6658 * t337;
    let t19733 = t2306 * t4422;
    let t19775 = t2365 * t56;
    let t19776 = t2118 * t19775;
    let t19803 = t19561 * t816;
    (t19562, t19563, t19637, t19693, t19733, t19775, t19776, t19803)
}
