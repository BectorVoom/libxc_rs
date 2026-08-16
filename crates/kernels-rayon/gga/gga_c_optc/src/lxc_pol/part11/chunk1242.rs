//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1242/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1242(t3360: f64, t48000: f64, t10004: f64, t2124: f64, t2126: f64, t2168: f64, t3491: f64, t3519: f64, t37853: f64, t48862: f64, t48866: f64, t48875: f64, t48904: f64, t56078: f64, t56124: f64, t56158: f64, t56166: f64, t56197: f64, t56205: f64, t56213: f64, t56232: f64, t686: f64, t695: f64, t696: f64, t705: f64, t7129: f64) -> (f64, f64) {
    let t56501 = t48000 * t3360;
    let t56521 = -0.18137053605011111023e1_f64 * t2168 * t56124 + 0.69545291918310062836e0_f64 * t2124 * t2126 * t56158 - 0.18137053605011111023e1_f64 * t2168 * t56078 + 0.16227234780939014661e2_f64 * t37853 + 0.14604511302845113196e2_f64 * t48862 + 0.24340852171408521992e1_f64 * t48866 + 0.8463958349005185144e0_f64 * t48875 - 0.62590762726479056552e1_f64 * t2124 * t7129 * t56501 - 0.15114211337509259186e-1_f64 * t695 * t696 * t56166 - 0.45342634012527777558e-1_f64 * t695 * t696 * t56205 - 0.1251815254529581131e2_f64 * t686 * t10004 * t56213 + 0.20863587575493018851e1_f64 * t686 * t3491 * t56232 + 0.60456845350037036744e0_f64 * t705 * t3519 * t56197 + 0.33855833396020740576e1_f64 * t48904;
    (t56501, t56521)
}
