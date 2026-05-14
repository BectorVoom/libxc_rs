//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 947/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk947<F: Float>(t1629: F, t760: F, t477: F, t5077: F, t6636: F, t332: F, t5094: F, t5083: F, t5084: F, t12684: F, t5095: F, t4103: F, t872: F, t132: F, t435: F, t4978: F) -> (F, F, F, F, F, F) {
    let t12790 = t760 * t1629;
    let t12794 = 2.0 / 15.0 * t5077 * t6636 * t12790 * t477;
    let t12795 = t12790 * t332;
    let t12798 = 2.0 / 15.0 * t5077 * t5094 * t12795;
    let t12801 = t5083 * t5084 * t12795 / 9.0;
    let t12803 = 4.0 / 15.0 * t12684 * t5095;
    let t12804 = t872 * t4103;
    let t12807 = t132 * t435 * t4978;
    (t12794, t12798, t12801, t12803, t12804, t12807)
}
