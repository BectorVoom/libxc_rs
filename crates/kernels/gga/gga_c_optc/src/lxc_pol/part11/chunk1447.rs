//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1447/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1447<F: Float>(t59160: F, t59162: F, t59165: F, t59169: F, t59171: F, t59173: F, t59176: F, t59179: F, t59181: F, t59183: F, t59186: F, t1220: F, t17440: F, t28010: F, t4230: F, t58426: F, t59188: F, t59191: F, t59193: F, t59196: F, t59199: F, t59202: F, t59205: F, t59209: F, t59212: F, t914: F) -> (F, F) {
    let t60252 = t59160 + t59162 - t59165 - t59169 - t59171 - t59173 + t59176 + t59179 + t59181 + t59183 - t59186;
    let t60259 = t59188 - t59191 + t59193 - t59196 - t59199 + t59202 + t59205 + t59209 + t59212 + F::new(140.0) / F::new(81.0) * t1220 * t914 * t28010 * t58426 - F::new(32.0) / F::new(3.0) * t4230 * t17440;
    (t60252, t60259)
}
