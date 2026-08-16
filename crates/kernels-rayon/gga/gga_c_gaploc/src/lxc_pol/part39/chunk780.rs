//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 780/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk780(t13225: f64, t943: f64, t2592: f64, t3511: f64, t1052: f64, t9767: f64, t3263: f64, t5559: f64, t977: f64, t1960: f64, t3322: f64, t3459: f64, t7324: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t13226 = t943 * t13225;
    let t13232 = t2592 * t3511;
    let t13234 = t9767 * t1052;
    let t13235 = t1052 * t3263;
    let t13237 = 6.0_f64 * t5559 * t13235;
    let t13238 = t3511 * t977;
    let t13239 = t1960 * t13238;
    let t13241 = t1052 * t3322;
    let t13243 = 2.0_f64 * t1960 * t13241;
    let t13245 = 4.0_f64 * t7324 * t3459;
    (t13226, t13232, t13234, t13235, t13237, t13238, t13239, t13241, t13243, t13245)
}
