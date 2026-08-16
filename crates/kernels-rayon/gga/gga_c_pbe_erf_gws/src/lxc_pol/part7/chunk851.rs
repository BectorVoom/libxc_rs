//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 851/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk851(t1371: f64, t553: f64, t6016: f64, t1331: f64, t8: f64, t147: f64, t551: f64, t6038: f64, t6041: f64, t1354: f64, t837: f64, t6006: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t16460 = t6016 * t1371 * t553;
    let t16463 = 1.0_f64 / t8 / t1331;
    let t16465 = t16463 * t147 * t551;
    let t16467 = 0.74395492895254307406e-5_f64 * t16465 * t553;
    let t16468 = t6038 * t553;
    let t16471 = 0.1035981803916141664e0_f64 * t6041 * t553;
    let t16474 = t837 * t1354 * t551 * t553;
    let t16477 = t6006 * t1371 * t553;
    (t16460, t16463, t16465, t16467, t16468, t16471, t16474, t16477)
}
