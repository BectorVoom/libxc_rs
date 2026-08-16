//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 573/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk573(t43: f64, t4360: f64, t476: f64, t4353: f64, t4356: f64, t261: f64, t52: f64, t1413: f64, t422: f64, zeta_threshold: f64) -> (f64, f64, f64, f64) {
    let t44 = t43 <= zeta_threshold;
    let t4361 = t476 * t4360;
    let t4364 = piecewise3(t44, 0.0_f64, 8.0_f64 / 27.0_f64 * t4353 - 2.0_f64 / 3.0_f64 * t4356 + 2.0_f64 / 3.0_f64 * t4361);
    let t4366 = 1.0_f64 / t52 / t261;
    let t4367 = t1413 * t422;
    (t4361, t4364, t4366, t4367)
}
