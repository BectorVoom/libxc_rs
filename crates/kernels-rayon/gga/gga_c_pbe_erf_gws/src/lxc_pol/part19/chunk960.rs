//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 960/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk960(t10843: f64, t593: f64, t2637: f64, t7130: f64, t3487: f64, t586: f64, t2615: f64, t2643: f64, t3421: f64, t610: f64, t5543: f64, t587: f64) -> (f64, f64, f64, f64, f64) {
    let t10845 = 8.0_f64 / 45.0_f64 * t10843 * t593;
    let t10847 = 8.0_f64 / 15.0_f64 * t7130 * t2637;
    let t10848 = t3487 * t586;
    let t10850 = 4.0_f64 / 45.0_f64 * t10848 * t593;
    let t10851 = t2615 * t2643;
    let t10852 = 16.0_f64 / 135.0_f64 * t10851;
    let t10853 = t3421 * t610;
    let t10854 = t5543 * t10853;
    let t10856 = 4.0_f64 / 27.0_f64 * t587 * t10854;
    (t10845, t10847, t10850, t10852, t10856)
}
