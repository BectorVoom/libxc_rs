//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 350/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk350(t1383: f64, t169: f64, t289: f64, t274: f64, t39: f64, t532: f64, t745: f64, t1216: f64, t1319: f64, t1322: f64) -> (f64, f64, f64, f64) {
    let t1386 = 0.31835665774679373271e-1_f64 * t169 * t289 * t1383;
    let t1388 = 0.3199504064530762818e0_f64 * t39 * t274;
    let t1389 = t532 * t745;
    let t1392 = t1319 * t1216 * t1322;
    (t1386, t1388, t1389, t1392)
}
