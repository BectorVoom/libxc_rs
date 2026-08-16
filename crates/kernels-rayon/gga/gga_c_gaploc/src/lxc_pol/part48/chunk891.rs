//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 891/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk891(t43363: f64, t43421: f64, t45170: f64, t45174: f64, t45176: f64, t45177: f64, t45178: f64, t45179: f64, t45180: f64, t45183: f64, t45187: f64, t45188: f64, t45192: f64, t45193: f64, t45194: f64, t45195: f64, t45197: f64, t45199: f64, t45200: f64) -> f64 {
    let t45202 = t45170 - t45174 - 0.38342925953920749677e1_f64 * t43363 - t45176 - t45177 - t45178 + t45179 - t45180 - t45183 + t45187 + t45188 + t45192 - t45193 - t45194 + t45195 + 0.38342925953920749677e0_f64 * t45197 - t45199 - t45200 - 0.23005755572352449806e1_f64 * t43421;
    t45202
}
