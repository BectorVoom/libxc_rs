//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1412/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1412(t1219: f64, t176: f64, t9337: f64, t27071: f64, t490: f64, t492: f64, t496: f64, t1186: f64, t1220: f64, t1221: f64, t1223: f64, t26256: f64, t26291: f64, t26302: f64, t26341: f64, t26849: f64, t26855: f64, t26857: f64, t2911: f64, t3274: f64, t3284: f64, t3980: f64, t8417: f64, t8422: f64, t8426: f64, t914: f64, t9221: f64) -> f64 {
    let t28088 = t176 * t9337 * t1219;
    let t28109 = 40.0_f64 / 81.0_f64 * t490 * t492 * t27071 * t496;
    let t28114 = t1220 * t914 * t1221 * t26341 / 6.0_f64 + 2.0_f64 / 3.0_f64 * t28088 * t1223 + 2.0_f64 / 3.0_f64 * t1220 * t914 * t3284 * t26291 - t1220 * t914 * t1221 * t26302 - 16.0_f64 / 3.0_f64 * t3274 * t8417 - 56.0_f64 / 9.0_f64 * t1220 * t914 * t8426 * t26256 + 4.0_f64 * t3274 * t8422 - t28109 - t26855 + t26857 - 0.10337952573961372198e-1_f64 * t3980 * t9221 * t2911 * t1186 - t26849;
    t28114
}
