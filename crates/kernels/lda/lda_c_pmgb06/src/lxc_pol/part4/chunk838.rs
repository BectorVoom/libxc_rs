//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 838/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk838<F: Float>(t2209: F, t73: F, t76: F, t1227: F, t2181: F, t1282: F, t2221: F, t342: F, t38: F, t776: F, t1234: F, t780: F) -> (F, F, F, F, F, F, F) {
    let t5721 = t73 * t2209;
    let t5731 = t76 * t2209;
    let t5737 = t2181 * t1227;
    let t5740 = t1282 * t2209;
    let t5749 = F::new(11.6921) * t38 * t2221 * t342;
    let t5752 = F::new(5.84605) * t38 * t776 * t1227;
    let t5755 = F::new(17.53815) * t38 * t780 * t1234;
    (t5721, t5731, t5737, t5740, t5749, t5752, t5755)
}
