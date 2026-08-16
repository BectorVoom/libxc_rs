//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 829/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk829(t185: f64, t7751: f64, t2730: f64, t2753: f64, t1639: f64, t649: f64, t1642: f64, t1730: f64, t1: f64, t837: f64, t2736: f64, t616: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t7753 = 8.0_f64 / 45.0_f64 * t185 * t7751;
    let t7757 = 16.0_f64 / 45.0_f64 * t2730 * t2753;
    let t7758 = t1639 * t649;
    let t7759 = t7758 * t1642;
    let t7775 = 16.0_f64 / 45.0_f64 * t1730 * t2753;
    let t7776 = t1 * t837;
    let t7777 = t7776 * t2736;
    let t7778 = t616 * t7777;
    (t7753, t7757, t7759, t7775, t7776, t7778)
}
