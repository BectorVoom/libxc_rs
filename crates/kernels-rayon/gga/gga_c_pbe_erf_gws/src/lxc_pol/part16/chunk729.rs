//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 729/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk729(t2: f64, t4516: f64, t39: f64, t784: f64, t799: f64, t1236: f64, t119: f64, t837: f64, t391: f64, t11: f64, t1246: f64, t398: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t4517 = t4516 * t2;
    let t4518 = t4517 * t39;
    let t4520 = t799 * t784;
    let t4521 = t1236 * t4520;
    let t4523 = t119 * t837;
    let t4524 = t391 * t4523;
    let t4527 = 1.0_f64/pow_3_2(t11);
    let t4528 = t4527 * t2;
    let t4529 = t4528 * t39;
    let t4531 = t1246 * t4520;
    let t4533 = t398 * t4523;
    (t4518, t4521, t4524, t4529, t4531, t4533)
}
