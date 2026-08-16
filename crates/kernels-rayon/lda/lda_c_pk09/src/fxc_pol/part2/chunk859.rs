//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 859/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk859(t849: f64, t8881: f64, t164: f64, t62: f64, t7766: f64, t896: f64, t890: f64, t192: f64, t200: f64, t4684: f64, t8049: f64, t8849: f64, t8851: f64, t8859: f64, t8863: f64, t8867: f64, t8871: f64, t8875: f64, t8879: f64) -> (f64, f64, f64) {
    let t8882 = t849 * t8881;
    let t8883 = t164 * t8882;
    let t8885 = t62 * t7766;
    let t8886 = t896 * t8885;
    let t8887 = t890 * t8886;
    let t8890 = -0.027433775686566395_f64 * t8849 - 0.027433775686566395_f64 * t8851 - 2.427516195194328_f64 * t200 * t8049 - 2.2140749178833072_f64 * t192 * t8049 + 1.800081713982063_f64 * t8859 - 1.800081713982063_f64 * t8863 - 1.800081713982063_f64 * t8867 + 22.07984838129906_f64 * t8871 + 22.07984838129906_f64 * t8875 + 22.07984838129906_f64 * t8879 + 22.07984838129906_f64 * t8883 + 1.800081713982063_f64 * t8887 - 1.2536914064583544_f64 * t4684;
    (t8883, t8887, t8890)
}
