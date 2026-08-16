//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 977/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk977(t1651: f64, t3526: f64, t587: f64, t7942: f64, t3465: f64, t661: f64, t5522: f64, t639: f64, t10524: f64, t2677: f64, t10535: f64, t7853: f64) -> (f64, f64, f64, f64, f64) {
    let t11037 = t1651 * t3526;
    let t11038 = t587 * t11037;
    let t11039 = 8.0_f64 / 135.0_f64 * t11038;
    let t11040 = 32.0_f64 / 135.0_f64 * t7942;
    let t11041 = t3465 * t661;
    let t11042 = t5522 * t11041;
    let t11044 = 4.0_f64 / 27.0_f64 * t639 * t11042;
    let t11045 = t2677 * t10524;
    let t11047 = 8.0_f64 / 9.0_f64 * t639 * t11045;
    let t11048 = t7853 * t10535;
    (t11039, t11040, t11044, t11047, t11048)
}
