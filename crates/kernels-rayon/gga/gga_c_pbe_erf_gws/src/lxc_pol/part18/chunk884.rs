//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 884/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk884(t43: f64, t476: f64, t9788: f64, t9779: f64, t9781: f64, t9784: f64, t3351: f64, t4366: f64, t422: f64, t1351: f64, t2485: f64, t1528: f64, t3354: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64) {
    let t44 = t43 <= zeta_threshold;
    let t9789 = t476 * t9788;
    let t9792 = piecewise3(t44, 0.0_f64, 8.0_f64 / 27.0_f64 * t9779 - 8.0_f64 / 9.0_f64 * t9781 - 2.0_f64 / 9.0_f64 * t9784 + 2.0_f64 / 3.0_f64 * t9789);
    let t9793 = t4366 * t3351;
    let t9794 = t9793 * t422;
    let t9796 = t2485 * t1351;
    let t9798 = t1528 * t3354;
    (t9789, t9792, t9794, t9796, t9798)
}
