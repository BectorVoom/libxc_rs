//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 673/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk673(t12290: f64, t12315: f64, t12317: f64, t12321: f64, t883: f64, t9198: f64, t2325: f64, t882: f64, t2321: f64, t3152: f64, t3148: f64, t135: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t12323 = t12290 + t12315 + t12317 + t12321;
    let t12351 = t883 * t9198;
    let t12352 = t2325 * t12351;
    let t12353 = t882 * t12352;
    let t12360 = t3152 * t2321;
    let t12361 = t882 * t12360;
    let t12366 = t3148 * t2321;
    let t12367 = t882 * t12366;
    let t12380 = 1.0_f64 / t135;
    (t12323, t12352, t12353, t12360, t12361, t12366, t12367, t12380)
}
