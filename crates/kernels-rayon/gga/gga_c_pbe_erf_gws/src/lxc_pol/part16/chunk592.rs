//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 592/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk592(t1697: f64, t954: f64, t422: f64, t1809: f64, t639: f64, t1640: f64, t219: f64, t1642: f64, t1044: f64, t626: f64, t1815: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t2672 = t1697 * t954;
    let t2673 = t2672 * t422;
    let t2674 = t1809 * t2673;
    let t2676 = 8.0_f64 / 45.0_f64 * t639 * t2674;
    let t2677 = t1640 * t219;
    let t2678 = t1642 * t954;
    let t2679 = t2678 * t422;
    let t2680 = t2677 * t2679;
    let t2682 = 4.0_f64 / 27.0_f64 * t639 * t2680;
    let t2683 = t1044 * t626;
    let t2684 = t2683 * t422;
    let t2685 = t1815 * t2684;
    (t2672, t2673, t2674, t2676, t2677, t2678, t2679, t2680, t2682, t2683, t2684, t2685)
}
