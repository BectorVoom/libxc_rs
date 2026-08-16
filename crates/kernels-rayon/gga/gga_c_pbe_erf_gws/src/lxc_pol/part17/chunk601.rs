//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 601/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk601(t1023: f64, t582: f64, t616: f64, t1018: f64, t185: f64, t1001: f64, t395: f64, t1758: f64, t2561: f64, t11: f64, t2555: f64, t571: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t2753 = t582 * t1023;
    let t2754 = t616 * t2753;
    let t2755 = 8.0_f64 / 45.0_f64 * t2754;
    let t2756 = t582 * t1018;
    let t2757 = t185 * t2756;
    let t2758 = 4.0_f64 / 45.0_f64 * t2757;
    let t2760 = t395 * t1001;
    let t2762 = t1758 * t2561;
    let t2763 = t11 * t2762;
    let t2765 = t571 * t2555;
    (t2753, t2755, t2756, t2758, t2760, t2762, t2763, t2765)
}
