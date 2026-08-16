//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 553/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk553(t140: f64, t1503: f64, t142: f64, t967: f64, t1516: f64, t485: f64, t971: f64, t395: f64, t102: f64, t481: f64, t974: f64, t2478: f64, t2481: f64, t2486: f64, t2489: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t2857 = t1503 * t140;
    let t2858 = t142 * t967;
    let t2862 = 0.48717083333333333333e0_f64 * t1516;
    let t2863 = t485 * t971;
    let t2864 = t2863 * t395;
    let t2865 = 0.48717083333333333333e0_f64 * t2864;
    let t2868 = 0.584605e1_f64 * t102 * t974 * t481;
    let t2873 = -t2478 / 9.0_f64 + 2.0_f64 / 3.0_f64 * t2481 - t2486 / 9.0_f64 - 2.0_f64 / 3.0_f64 * t2489;
    (t2857, t2858, t2862, t2863, t2864, t2865, t2868, t2873)
}
