//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1205/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1205(t1109: f64, t810: f64, t2306: f64, t35187: f64, t3074: f64, t831: f64, t9807: f64, t2395: f64, t3717: f64, t1144: f64, t858: f64, t1105: f64, t8749: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t35207 = t1109 * t810;
    let t35259 = t2306 * t35187;
    let t35260 = t3074 * t35259;
    let t35428 = t831 * t9807;
    let t35433 = t2395 * t3717;
    let t35566 = t858 * t1144;
    let t35654 = t8749 * t1105;
    (t35207, t35260, t35428, t35433, t35566, t35654)
}
