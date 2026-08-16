//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 964/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk964(t496: f64, t8545: f64, t492: f64, t490: f64, t1210: f64, t2839: f64, t1188: f64, t1220: f64, t1223: f64, t277: f64, t3274: f64, t3281: f64, t3286: f64, t8410: f64, t8417: f64, t8422: f64, t8431: f64, t8436: f64, t8444: f64, t9221: f64, t95: f64) -> (f64, f64) {
    let t9226 = t8545 * t496;
    let t9227 = t492 * t9226;
    let t9229 = 5.0_f64 / 27.0_f64 * t490 * t9227;
    let t9230 = t1210 * t2839;
    let t9232 = t8410 * t1223 / 2.0_f64 - 4.0_f64 / 3.0_f64 * t1220 * t8417 + t1220 * t8422 + 14.0_f64 / 27.0_f64 * t1220 * t8431 + t1220 * t8436 / 6.0_f64 + t3274 * t3281 / 2.0_f64 + 2.0_f64 / 3.0_f64 * t3274 * t3286 + t8444 / 6.0_f64 + 0.25844881434903430496e-2_f64 * t95 * t277 * t9221 * t1188 + t9229 - t9230 / 3.0_f64;
    (t9227, t9232)
}
