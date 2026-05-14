//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 762/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk762<F: Float>(t844: F, t8873: F, t164: F, t2149: F, t650: F, t849: F, t61: F, t7704: F, t62: F, t7766: F, t896: F, t890: F, t192: F, t200: F, t4684: F, t8049: F, t8849: F, t8851: F, t8859: F, t8863: F, t8867: F, t8871: F) -> (F, F, F, F, F) {
    let t8874 = t844 * t8873;
    let t8875 = t164 * t8874;
    let t8877 = t650 * t2149;
    let t8878 = t849 * t8877;
    let t8879 = t164 * t8878;
    let t8881 = t61 * t7704;
    let t8882 = t849 * t8881;
    let t8883 = t164 * t8882;
    let t8885 = t62 * t7766;
    let t8886 = t896 * t8885;
    let t8887 = t890 * t8886;
    let t8890 = -0.027433775686566395 * t8849 - 0.027433775686566395 * t8851 - 2.427516195194328 * t200 * t8049 - 2.2140749178833072 * t192 * t8049 + 1.800081713982063 * t8859 - 1.800081713982063 * t8863 - 1.800081713982063 * t8867 + 22.07984838129906 * t8871 + 22.07984838129906 * t8875 + 22.07984838129906 * t8879 + 22.07984838129906 * t8883 + 1.800081713982063 * t8887 - 1.2536914064583544 * t4684;
    (t8875, t8879, t8883, t8887, t8890)
}
