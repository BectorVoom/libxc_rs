//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 966/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk966(t3274: f64, t3277: f64, t1222: f64, t7274: f64, t1220: f64, t1186: f64, t277: f64, t2911: f64, t3268: f64, t3290: f64, t3980: f64, t4281: f64, t8552: f64, t8695: f64, t9234: f64, t9237: f64, t9241: f64, t9244: f64, t9251: f64, t9254: f64, t95: f64) -> (f64, f64) {
    let t9258 = t3274 * t3277;
    let t9260 = t7274 * t1222;
    let t9261 = t1220 * t9260;
    let t9263 = 2.0_f64 / 9.0_f64 * t9234 - t9237 / 3.0_f64 - t3274 * t3290 - t4281 * t9241 + 2.0_f64 / 3.0_f64 * t4281 * t9244 - 0.77534644304710291488e-2_f64 * t3980 * t1186 * t2911 * t3268 + t8552 + 0.51689762869806860992e-2_f64 * t95 * t277 * t9251 * t9254 + t9258 / 3.0_f64 - t9261 / 9.0_f64 - t8695;
    (t9260, t9263)
}
