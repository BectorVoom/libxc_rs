//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 603/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk603(t2768: f64, t571: f64, t2704: f64, t1014: f64, t401: f64, t1856: f64, t2561: f64, t2555: f64, t606: f64, t1756: f64, t1844: f64, t1851: f64, t1852: f64, t25: f64, t2718: f64, t2760: f64, t2763: f64, t2766: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t2769 = t571 * t2768;
    let t2770 = t2704 * t2769;
    let t2773 = t401 * t1014;
    let t2775 = t1856 * t2561;
    let t2778 = t606 * t2555;
    let t2781 = t606 * t2768;
    let t2784 = t1844 + 0.11997222222222222222e-1_f64 * t1756 + 0.11997222222222222222e-1_f64 * t2760 - 0.23994444444444444445e-1_f64 * t2763 + 0.71983333333333333334e-1_f64 * t2766 - 0.71983333333333333334e-1_f64 * t2770 + t1851 + 0.44444444444444444445e-2_f64 * t1852 + 0.44444444444444444445e-2_f64 * t2773 - 0.22222222222222222222e-2_f64 * t25 * t2775 + 0.13333333333333333333e-1_f64 * t25 * t2778 - 0.13333333333333333333e-1_f64 * t2718 * t2781;
    (t2769, t2770, t2775, t2778, t2781, t2784)
}
