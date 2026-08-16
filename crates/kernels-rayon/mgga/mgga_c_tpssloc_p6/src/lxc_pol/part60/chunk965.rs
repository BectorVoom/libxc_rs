//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 965/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk965(t1888: f64, t32862: f64, t86873: f64, t118632: f64, t23270: f64, t25169: f64, t5636: f64, t22986: f64, t30622: f64, t5544: f64, t118649: f64, t118532: f64, t32844: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t126264 = 0.6579736267392905746e-1_f64 * t1888 * t86873 * t32862;
    let t126278 = 0.3289868133696452873e-1_f64 * t118632;
    let t126286 = 0.9869604401089358619e-1_f64 * t1888 * t23270 * t25169 * t5636;
    let t126290 = 0.3289868133696452873e-1_f64 * t22986 * t23270 * t30622 * t5544;
    let t126291 = 0.15352717957250113407e0_f64 * t118649;
    let t126294 = t118532 * t32844;
    (t126264, t126278, t126286, t126290, t126291, t126294)
}
