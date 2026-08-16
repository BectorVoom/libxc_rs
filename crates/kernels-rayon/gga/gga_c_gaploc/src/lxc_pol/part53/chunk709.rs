//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 709/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk709(t13235: f64, t5559: f64, t1052: f64, t3322: f64, t1960: f64, t3459: f64, t7324: f64, t2321: f64, t3701: f64, t882: f64, t11981: f64, t874: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t13237 = 6.0_f64 * t5559 * t13235;
    let t13241 = t1052 * t3322;
    let t13243 = 2.0_f64 * t1960 * t13241;
    let t13245 = 4.0_f64 * t7324 * t3459;
    let t13725 = t3701 * t2321;
    let t13726 = t882 * t13725;
    let t13728 = t11981 * t874;
    (t13237, t13241, t13243, t13245, t13725, t13726, t13728)
}
