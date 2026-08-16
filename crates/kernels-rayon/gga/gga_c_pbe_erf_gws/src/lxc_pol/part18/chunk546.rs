//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 546/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk546(t2790: f64, t564: f64, t1006: f64, t612: f64, t1883: f64, t582: f64, t996: f64, t561: f64, t198: f64, t34: f64, t2735: f64, t1046: f64, t633: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t2792 = 4.0_f64 / 15.0_f64 * t2790 * t564;
    let t2794 = 2.0_f64 / 15.0_f64 * t1006 * t612;
    let t2795 = 8.0_f64 / 45.0_f64 * t1883;
    let t2796 = t582 * t996;
    let t2797 = t561 * t2796;
    let t2798 = 8.0_f64 / 45.0_f64 * t2797;
    let t2799 = t198 * t34;
    let t2800 = t2735 * t2799;
    let t2802 = 4.0_f64 / 15.0_f64 * t561 * t2800;
    let t2806 = 2.0_f64 / 15.0_f64 * t633 * t1046;
    (t2792, t2794, t2795, t2796, t2797, t2798, t2799, t2800, t2802, t2806)
}
