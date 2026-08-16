//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1165/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1165(t23789: f64, t7341: f64, t837: f64, t845: f64, t7214: f64, t8280: f64, t2477: f64, t7337: f64, t1015: f64, t2247: f64, t2364: f64, t24187: f64, t24190: f64, t24192: f64, t24197: f64, t24202: f64, t2569: f64, t3980: f64, t7182: f64, t7259: f64, t8267: f64, t8287: f64, t8289: f64, t8298: f64, t960: f64) -> (f64, f64, f64) {
    let t24206 = 0.1403573615389248977e2_f64 * t845 * t7341 * t23789 * t837;
    let t24212 = t8280 * t7214;
    let t24215 = 0.1038945353962551798e3_f64 * t7337 * t2477;
    let t24220 = -4.0_f64 / 3.0_f64 * t24187 - 400.0_f64 / 81.0_f64 * t24190 + 16000000.0_f64 / 243.0_f64 * t8287 * t24192 * t8289 * t8298 + 8.0_f64 / 9.0_f64 * t24197 + t24202 + t24206 - 448.0_f64 / 81.0_f64 * t2364 * t7259 - 100.0_f64 * t7182 * t2247 * t1015 + 400.0_f64 / 81.0_f64 * t24212 - t24215 - 0.10337952573961372198e-1_f64 * t3980 * t8267 * t2569 * t960;
    (t24206, t24215, t24220)
}
