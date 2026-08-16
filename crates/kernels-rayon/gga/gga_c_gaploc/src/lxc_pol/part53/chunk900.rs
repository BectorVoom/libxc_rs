//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 900/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk900(t2508: f64, t2927: f64, t3270: f64, t13221: f64, t7137: f64, t13217: f64, t13185: f64, t40877: f64, t1897: f64, t28720: f64, t9014: f64, t28024: f64, t2936: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t43263 = 0.76905262301422242837e-2_f64 * t2508 * t3270 * t2927;
    let t43265 = 0.10254034973522965712e-1_f64 * t7137 * t13221;
    let t43267 = 0.61524209841137794268e-1_f64 * t7137 * t13217;
    let t43269 = 0.71778244814660759981e-1_f64 * t7137 * t13185;
    let t43274 = 0.85450291446024714264e-3_f64 * t40877;
    let t43282 = 0.92286314761706691403e-1_f64 * t1897 * t9014 * t28720;
    let t43286 = 0.53833683610995569986e-1_f64 * t2508 * t2936 * t28024;
    (t43263, t43265, t43267, t43269, t43274, t43282, t43286)
}
