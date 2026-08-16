//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 783/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk783(t299: f64, t481: f64, t799: f64, t5761: f64, t4516: f64, t103: f64, t2: f64, t39: f64, t497: f64, t542: f64, t496: f64, t120: f64, t1508: f64, t19: f64, param_hyb_omega_0: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t5763 = t799 * t299 * t481;
    let t5764 = t5761 * t5763;
    let t5772 = param_hyb_omega_0 * t4516;
    let t5773 = t103 * t2;
    let t5776 = 0.19486833333333333333e1_f64 * t5772 * t5773 * t39;
    let t5783 = t542 * t497;
    let t5784 = t496 * t5783;
    let t5795 = t1508 * t120 * t19;
    (t5763, t5764, t5776, t5783, t5784, t5795)
}
