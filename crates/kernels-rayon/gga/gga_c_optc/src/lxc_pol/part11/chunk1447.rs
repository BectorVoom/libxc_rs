//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1447/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1447(t59160: f64, t59162: f64, t59165: f64, t59169: f64, t59171: f64, t59173: f64, t59176: f64, t59179: f64, t59181: f64, t59183: f64, t59186: f64, t1220: f64, t17440: f64, t28010: f64, t4230: f64, t58426: f64, t59188: f64, t59191: f64, t59193: f64, t59196: f64, t59199: f64, t59202: f64, t59205: f64, t59209: f64, t59212: f64, t914: f64) -> (f64, f64) {
    let t60252 = t59160 + t59162 - t59165 - t59169 - t59171 - t59173 + t59176 + t59179 + t59181 + t59183 - t59186;
    let t60259 = t59188 - t59191 + t59193 - t59196 - t59199 + t59202 + t59205 + t59209 + t59212 + 140.0_f64 / 81.0_f64 * t1220 * t914 * t28010 * t58426 - 32.0_f64 / 3.0_f64 * t4230 * t17440;
    (t60252, t60259)
}
