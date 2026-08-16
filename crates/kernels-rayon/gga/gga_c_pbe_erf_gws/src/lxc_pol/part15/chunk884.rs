//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 884/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk884(t1620: f64, t7653: f64, t1821: f64, t7359: f64, t587: f64, t1000: f64, t1804: f64, t5548: f64, t2688: f64, t5129: f64, t2555: f64, t5125: f64) -> (f64, f64, f64, f64, f64) {
    let t7655 = 16.0_f64 / 45.0_f64 * t1620 * t7653;
    let t7656 = t1821 * t7359;
    let t7658 = 8.0_f64 / 15.0_f64 * t587 * t7656;
    let t7659 = t1000 * t1804;
    let t7660 = t5548 * t7659;
    let t7662 = 8.0_f64 / 45.0_f64 * t587 * t7660;
    let t7663 = t5129 * t2688;
    let t7665 = 16.0_f64 / 135.0_f64 * t587 * t7663;
    let t7666 = t5125 * t2555;
    (t7655, t7658, t7662, t7665, t7666)
}
