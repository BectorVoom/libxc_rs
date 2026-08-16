//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 953/1049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk953(t118821: f64, t1527: f64, t1888: f64, t23270: f64, t1880: f64, t28263: f64, t30663: f64, t32862: f64, t86873: f64, t118632: f64, t25169: f64, t5636: f64) -> (f64, f64, f64, f64, f64) {
    let t126246 = 0.6579736267392905746e-1_f64 * t1888 * t23270 * t118821 * t1527;
    let t126249 = 0.16449340668482264365e-1_f64 * t1880 * t30663 * t28263;
    let t126264 = 0.6579736267392905746e-1_f64 * t1888 * t86873 * t32862;
    let t126278 = 0.3289868133696452873e-1_f64 * t118632;
    let t126286 = 0.9869604401089358619e-1_f64 * t1888 * t23270 * t25169 * t5636;
    (t126246, t126249, t126264, t126278, t126286)
}
