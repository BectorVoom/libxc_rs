//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 605/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk605(t2796: f64, t561: f64, t198: f64, t34: f64, t2735: f64, t1046: f64, t633: f64, t1006: f64, t583: f64, t1689: f64, t1743: f64, t2696: f64, t2699: f64, t2702: f64, t2707: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t2797 = t561 * t2796;
    let t2798 = 8.0_f64 / 45.0_f64 * t2797;
    let t2799 = t198 * t34;
    let t2800 = t2735 * t2799;
    let t2802 = 4.0_f64 / 15.0_f64 * t561 * t2800;
    let t2806 = 2.0_f64 / 15.0_f64 * t633 * t1046;
    let t2807 = t1006 * t583;
    let t2808 = 4.0_f64 / 45.0_f64 * t2807;
    let t2814 = -t1743 - 0.62972222222222222223e-3_f64 * t1689 - 0.62972222222222222223e-3_f64 * t2696 + 0.12594444444444444445e-2_f64 * t2699 - 0.37783333333333333334e-2_f64 * t2702 - 0.37783333333333333334e-2_f64 * t2707;
    (t2798, t2799, t2800, t2802, t2806, t2808, t2814)
}
