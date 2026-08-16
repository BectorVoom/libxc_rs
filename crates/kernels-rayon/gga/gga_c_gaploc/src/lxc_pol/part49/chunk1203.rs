//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 1203/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk1203(t42315: f64, t42316: f64, t42350: f64, t42354: f64, t48131: f64, t48134: f64, t48137: f64, t48140: f64, t48141: f64, t48142: f64, t48143: f64, t48144: f64) -> f64 {
    let t48146 = 0.11502877786176224903e2_f64 * t48131 + 0.11502877786176224903e2_f64 * t48134 + 0.11502877786176224903e2_f64 * t48137 - t42315 - 0.14896037479937677779e-1_f64 * t42316 + t48140 + t48141 - t48142 + t48143 - t42350 + 0.71500979903700853338e0_f64 * t48144 + t42354;
    t48146
}
