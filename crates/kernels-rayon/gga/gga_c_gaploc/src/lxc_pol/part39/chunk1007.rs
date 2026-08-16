//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 1007/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk1007(t1897: f64, t28720: f64, t9014: f64, t40896: f64, t2508: f64, t28024: f64, t2936: f64, t40898: f64, t40900: f64, t13176: f64, t731: f64, t22090: f64, t28668: f64, t8604: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t43282 = 0.92286314761706691403e-1_f64 * t1897 * t9014 * t28720;
    let t43283 = 0.17090058289204942853e-2_f64 * t40896;
    let t43286 = 0.53833683610995569986e-1_f64 * t2508 * t2936 * t28024;
    let t43288 = 0.85450291446024714264e-3_f64 * t40898;
    let t43289 = 0.85450291446024714264e-3_f64 * t40900;
    let t43290 = t731 * t13176;
    let t43295 = 0.1845726295234133828e0_f64 * t2508 * t22090 * t8604 * t28668;
    (t43282, t43283, t43286, t43288, t43289, t43290, t43295)
}
