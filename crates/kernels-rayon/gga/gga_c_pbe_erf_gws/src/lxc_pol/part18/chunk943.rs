//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 943/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk943(t3456: f64, t579: f64, t1033: f64, t2749: f64, t3513: f64, t7011: f64, t4913: f64, t2607: f64, t2722: f64, t1621: f64, t1620: f64, t2603: f64, t2612: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10616 = 4.0_f64 / 15.0_f64 * t579 * t3456;
    let t10617 = t1033 * t2749;
    let t10618 = 8.0_f64 / 45.0_f64 * t10617;
    let t10620 = 8.0_f64 / 15.0_f64 * t7011 * t3513;
    let t10622 = 8.0_f64 / 15.0_f64 * t4913 * t3513;
    let t10623 = t2607 * t2722;
    let t10624 = t1621 * t10623;
    let t10626 = 8.0_f64 / 15.0_f64 * t1620 * t10624;
    let t10628 = 8.0_f64 / 15.0_f64 * t2612 * t2603;
    (t10616, t10618, t10620, t10622, t10626, t10628)
}
