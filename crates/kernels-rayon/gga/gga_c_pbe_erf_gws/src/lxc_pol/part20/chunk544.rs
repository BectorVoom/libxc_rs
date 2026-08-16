//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 544/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk544(t2768: f64, t606: f64, t1756: f64, t1844: f64, t1851: f64, t1852: f64, t25: f64, t2718: f64, t2760: f64, t2763: f64, t2766: f64, t2770: f64, t2773: f64, t2775: f64, t2778: f64) -> (f64, f64) {
    let t2781 = t606 * t2768;
    let t2784 = t1844 + 0.11997222222222222222e-1_f64 * t1756 + 0.11997222222222222222e-1_f64 * t2760 - 0.23994444444444444445e-1_f64 * t2763 + 0.71983333333333333334e-1_f64 * t2766 - 0.71983333333333333334e-1_f64 * t2770 + t1851 + 0.44444444444444444445e-2_f64 * t1852 + 0.44444444444444444445e-2_f64 * t2773 - 0.22222222222222222222e-2_f64 * t25 * t2775 + 0.13333333333333333333e-1_f64 * t25 * t2778 - 0.13333333333333333333e-1_f64 * t2718 * t2781;
    (t2781, t2784)
}
