//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 777/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk777(t13182: f64, t9647: f64, t1029: f64, t3276: f64, t2508: f64, t3433: f64, t954: f64, t3251: f64, t9014: f64, t10628: f64, t5539: f64, t12605: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t13183 = t9647 * t13182;
    let t13184 = 0.64087718584518535698e-3_f64 * t13183;
    let t13185 = t3276 * t1029;
    let t13187 = 0.53833683610995569986e-1_f64 * t2508 * t13185;
    let t13188 = t954 * t3433;
    let t13189 = t2508 * t13188;
    let t13191 = t9014 * t3251;
    let t13193 = 0.92286314761706691403e-1_f64 * t2508 * t13191;
    let t13194 = t5539 * t10628;
    let t13195 = t9647 * t13194;
    let t13196 = 0.12817543716903707139e-2_f64 * t13195;
    let t13197 = 0.1922631557535556071e-2_f64 * t12605;
    (t13184, t13185, t13187, t13188, t13189, t13191, t13193, t13194, t13196, t13197)
}
