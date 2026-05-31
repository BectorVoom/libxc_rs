//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1165/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1165<F: Float>(t23789: F, t7341: F, t837: F, t845: F, t7214: F, t8280: F, t2477: F, t7337: F, t1015: F, t2247: F, t2364: F, t24187: F, t24190: F, t24192: F, t24197: F, t24202: F, t2569: F, t3980: F, t7182: F, t7259: F, t8267: F, t8287: F, t8289: F, t8298: F, t960: F) -> (F, F, F) {
    let t24206 = F::cast_from(0.1403573615389248977e2_f64) * t845 * t7341 * t23789 * t837;
    let t24212 = t8280 * t7214;
    let t24215 = F::cast_from(0.1038945353962551798e3_f64) * t7337 * t2477;
    let t24220 = -F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t24187 - F::cast_from(400.0_f64) / F::cast_from(81.0_f64) * t24190 + F::cast_from(16000000.0_f64) / F::cast_from(243.0_f64) * t8287 * t24192 * t8289 * t8298 + F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t24197 + t24202 + t24206 - F::cast_from(448.0_f64) / F::cast_from(81.0_f64) * t2364 * t7259 - F::cast_from(100.0_f64) * t7182 * t2247 * t1015 + F::cast_from(400.0_f64) / F::cast_from(81.0_f64) * t24212 - t24215 - F::cast_from(0.10337952573961372198e-1_f64) * t3980 * t8267 * t2569 * t960;
    (t24206, t24215, t24220)
}
