//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 657/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk657<F: Float>(t2160: F, t643: F, t2158: F, t638: F, t248: F, t3662: F, t3672: F, t3678: F, t3700: F, t4483: F, t4485: F, t4516: F, t4518: F, t4520: F, t4522: F, t4525: F, t4527: F, t4531: F, t4532: F) -> (F,) {
    let t4534 = t643 * t2160;
    let t4537 = 8.0 * t638 * t2158;
    let t4538 = t4483 - t4485 + t248 * t4516 + 8.0 * t4518 + 12.0 * t4520 + 20.0 * t4522 + t4525 + 0.0004883052614935079 * t3662 - 32.0 * t4527 + t3672 - t3678 + t3700 - t4531 + 0.00024415263074675396 * t4532 - 8.0 * t4534 + t4537;
    (t4538,)
}
