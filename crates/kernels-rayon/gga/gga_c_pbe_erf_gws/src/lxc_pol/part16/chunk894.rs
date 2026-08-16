//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 894/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk894(t185: f64, t7751: f64, t1033: f64, t1795: f64, t2730: f64, t2753: f64, t1639: f64, t649: f64, t1642: f64, t7506: f64, t7115: f64, t4908: f64, t616: f64) -> (f64, f64, f64, f64, f64) {
    let t7753 = 8.0_f64 / 45.0_f64 * t185 * t7751;
    let t7755 = 4.0_f64 / 15.0_f64 * t1033 * t1795;
    let t7757 = 16.0_f64 / 45.0_f64 * t2730 * t2753;
    let t7758 = t1639 * t649;
    let t7759 = t7758 * t1642;
    let t7760 = t7759 * t7506;
    let t7762 = 8.0_f64 / 27.0_f64 * t7115 * t7760;
    let t7764 = 4.0_f64 / 15.0_f64 * t616 * t4908;
    (t7753, t7755, t7757, t7762, t7764)
}
