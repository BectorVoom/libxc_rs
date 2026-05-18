//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 858/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk858<F: Float>(t8865: F, t891: F, t890: F, t2143: F, t650: F, t844: F, t164: F, t61: F, t7766: F, t2149: F, t849: F, t7704: F) -> (F, F, F, F, F) {
    let t8866 = t891 * t8865;
    let t8867 = t890 * t8866;
    let t8869 = t650 * t2143;
    let t8870 = t844 * t8869;
    let t8871 = t164 * t8870;
    let t8873 = t61 * t7766;
    let t8874 = t844 * t8873;
    let t8875 = t164 * t8874;
    let t8877 = t650 * t2149;
    let t8878 = t849 * t8877;
    let t8879 = t164 * t8878;
    let t8881 = t61 * t7704;
    (t8867, t8871, t8875, t8879, t8881)
}
