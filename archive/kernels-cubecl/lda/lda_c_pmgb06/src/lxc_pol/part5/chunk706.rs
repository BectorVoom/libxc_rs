//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 706/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk706<F: Float>(t439: F, t6465: F, t4148: F, t4151: F, t5104: F, t5107: F, t5114: F, t5117: F, t5126: F, t6445: F, t6447: F, t6451: F, t6453: F, t6455: F, t6457: F, t6459: F, t6463: F) -> (F, F) {
    let t6467 = t439 * t6465 / F::cast_from(27.0_f64);
    let t6468 = t6445 + t6447 + t6451 - t6453 + F::cast_from(2.0_f64) / F::cast_from(135.0_f64) * t4148 - t4151 - t5104 - t5107 - t5114 - t5117 - t5126 + t6455 - t6457 - t6459 - t6463 - t6467;
    (t6467, t6468)
}
