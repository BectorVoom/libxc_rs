//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 726/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk726(t1211: f64, t2053: f64, t1123: f64, t3950: f64, t850: f64, t833: f64, t2409: f64, t3050: f64, t3959: f64, t1146: f64, t1173: f64, t3045: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t4120 = t1211 * t2053;
    let t4127 = t850 * t1123 * t3950;
    let t4128 = t4127 * t833;
    let t4130 = t2409 * t3050;
    let t4131 = t3959 * t4130;
    let t4133 = t1173 * t1146;
    let t4135 = t2409 * t3045;
    (t4120, t4127, t4128, t4130, t4131, t4133, t4135)
}
