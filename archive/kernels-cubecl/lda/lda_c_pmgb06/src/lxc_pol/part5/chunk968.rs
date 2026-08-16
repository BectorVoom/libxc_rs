//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 968/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk968<F: Float>(t12912: F, t835: F, t1977: F, t5194: F, t1983: F, t5210: F, t830: F, t2462: F, t3226: F, t1447: F, t6533: F, t486: F, t6843: F) -> (F, F, F, F, F, F) {
    let t15831 = t12912 * t835;
    let t15835 = t5194 * t1977;
    let t15838 = t830 * t5210 * t1983;
    let t15840 = t3226 * t2462;
    let t15842 = t1447 * t6533;
    let t15850 = t486 * t6843;
    (t15831, t15835, t15838, t15840, t15842, t15850)
}
