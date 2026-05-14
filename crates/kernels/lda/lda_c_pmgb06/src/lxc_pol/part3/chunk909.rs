//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 909/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk909<F: Float>(t1444: F, t5337: F, t12174: F, t12179: F, t12184: F, t12186: F, t12189: F, t12192: F, t12197: F, t12199: F, t12201: F, t12203: F, t12208: F, t132: F, t137: F, t1629: F, t4815: F) -> (F, F, F) {
    let t12210 = 2.0 / 15.0 * t1444 * t5337;
    let t12211 = -t12174 - t12179 - t12184 - t12186 - t12189 + t12192 - t12197 - t12199 - t12201 - t12203 + t12208 + t12210;
    let t12219 = t132 * t137 * t4815 * t1629 / 10.0;
    (t12210, t12211, t12219)
}
