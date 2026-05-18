//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1201/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1201<F: Float>(t1977: F, t5194: F, t1983: F, t5210: F, t830: F, t2462: F, t3226: F, t1447: F, t6533: F, t131: F, t4238: F, t160: F) -> (F, F, F, F, F, F) {
    let t15835 = t5194 * t1977;
    let t15836 = F::new(8.0) / F::new(135.0) * t15835;
    let t15838 = t830 * t5210 * t1983;
    let t15839 = F::new(4.0) / F::new(27.0) * t15838;
    let t15840 = t3226 * t2462;
    let t15841 = F::new(8.0) / F::new(135.0) * t15840;
    let t15842 = t1447 * t6533;
    let t15843 = F::new(8.0) / F::new(135.0) * t15842;
    let t15844 = t131 * t4238;
    let t15845 = t160 * t15844;
    (t15836, t15839, t15841, t15843, t15844, t15845)
}
