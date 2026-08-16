//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2277/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2277(t24600: f64, t24615: f64, t1090: f64, t12648: f64, t14165: f64, t2128: f64, t24589: f64, t24590: f64, t24601: f64, t24603: f64, t27411: f64, t27433: f64, t27549: f64, t27774: f64, t4728: f64, t5059: f64, t7287: f64, t85661: f64, t85669: f64, t86403: f64, t94349: f64, t94354: f64, t94358: f64, t94363: f64, t94365: f64, t94369: f64, t94374: f64) -> f64 {
    let t94378 = t24600 * t24615;
    let t94385 = -0.3289868133696452873e-1_f64 * t2128 * t24590 * t27411 + 0.36554090374405031923e-2_f64 * t27549 * t24601 * t27774 * t12648 + 0.21932454224643019154e-1_f64 * t27549 * t24601 * t94349 * t14165 + 0.54831135561607547884e-2_f64 * t24589 * t94354 * t7287 + 0.54831135561607547884e-2_f64 * t24589 * t94358 * t7287 + t94363 + t94365 - 0.54831135561607547884e-2_f64 * t24589 * t86403 * t27433 + 0.10966227112321509577e-1_f64 * t24589 * t94369 * t4728 * t24603 + 0.54831135561607547884e-2_f64 * t24589 * t94374 * t7287 - 0.10966227112321509577e-1_f64 * t24589 * t94378 * t5059 * t1090 + 0.36554090374405031922e-2_f64 * t85661 - 0.27415567780803773942e-2_f64 * t85669;
    t94385
}
