//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1070/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1070(t19288: f64, t5795: f64, t119: f64, t481: f64, t837: f64, t1513: f64, t1365: f64, t497: f64, t496: f64, t1548: f64, t542: f64, t156: f64, t5790: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t19289 = t5795 * t19288;
    let t19290 = 0.19486833333333333333e1_f64 * t19289;
    let t19292 = t119 * t837 * t481;
    let t19293 = t1513 * t19292;
    let t19294 = 0.60625703703703703703e1_f64 * t19293;
    let t19295 = t1365 * t497;
    let t19296 = t496 * t19295;
    let t19298 = t542 * t1548;
    let t19299 = t496 * t19298;
    let t19301 = t156 * t5790;
    (t19290, t19292, t19294, t19295, t19296, t19298, t19299, t19301)
}
