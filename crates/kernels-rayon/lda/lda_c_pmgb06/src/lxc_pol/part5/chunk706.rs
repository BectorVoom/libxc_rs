//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 706/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk706(t439: f64, t6465: f64, t4148: f64, t4151: f64, t5104: f64, t5107: f64, t5114: f64, t5117: f64, t5126: f64, t6445: f64, t6447: f64, t6451: f64, t6453: f64, t6455: f64, t6457: f64, t6459: f64, t6463: f64) -> (f64, f64) {
    let t6467 = t439 * t6465 / 27.0_f64;
    let t6468 = t6445 + t6447 + t6451 - t6453 + 2.0_f64 / 135.0_f64 * t4148 - t4151 - t5104 - t5107 - t5114 - t5117 - t5126 + t6455 - t6457 - t6459 - t6463 - t6467;
    (t6467, t6468)
}
