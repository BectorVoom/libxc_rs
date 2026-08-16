//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 649/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk649(t1222: f64, t2367: f64, t1220: f64, t1221: f64, t2860: f64, t914: f64, t3086: f64, t496: f64, t2850: f64, t2856: f64, t1188: f64, t1223: f64, t277: f64, t2990: f64, t2997: f64, t3015: f64, t3023: f64, t3125: f64, t3268: f64, t3274: f64, t95: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3277 = t2367 * t1222;
    let t3278 = t1220 * t3277;
    let t3280 = t1221 * t2860;
    let t3281 = t914 * t3280;
    let t3284 = t3086 * t496;
    let t3285 = t3284 * t2850;
    let t3286 = t914 * t3285;
    let t3289 = t1221 * t2856;
    let t3290 = t914 * t3289;
    let t3293 = 0.25844881434903430496e-2_f64 * t95 * t277 * t3268 * t1188 - t2997 + t2990 + t3015 + t3023 + t3125 + t3274 * t1223 / 3.0_f64 + t3278 / 9.0_f64 + t1220 * t3281 / 6.0_f64 + 2.0_f64 / 9.0_f64 * t1220 * t3286 - t1220 * t3290 / 3.0_f64;
    (t3277, t3280, t3281, t3284, t3285, t3286, t3289, t3290, t3293)
}
