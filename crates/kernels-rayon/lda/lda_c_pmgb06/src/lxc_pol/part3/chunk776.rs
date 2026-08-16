//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 776/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk776(t439: f64, t5291: f64, t1868: f64, t477: f64, t1385: f64, t2010: f64, t5244: f64, t5247: f64, t5250: f64, t5252: f64, t5256: f64, t5259: f64, t5263: f64, t5266: f64, t5270: f64, t5275: f64, t5279: f64, t5284: f64, t5286: f64, t5289: f64) -> (f64, f64, f64, f64, f64) {
    let t5293 = t439 * t5291 / 45.0_f64;
    let t5294 = t1868 * t477;
    let t5295 = t1385 * t5294;
    let t5297 = 4.0_f64 / 45.0_f64 * t2010 * t5295;
    let t5298 = -t5244 - t5247 - t5250 + t5252 + t5256 + t5259 + t5263 + t5266 - t5270 - t5275 - t5279 - t5284 - t5286 - t5289 - t5293 - t5297;
    (t5293, t5294, t5295, t5297, t5298)
}
