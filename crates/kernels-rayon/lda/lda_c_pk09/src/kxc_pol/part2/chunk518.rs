//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 518/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk518(t3086: f64, t61: f64, t96: f64, t839: f64, t62: f64, t891: f64, t917: f64, t127: f64, t567: f64, t126: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t3088 = t96 * t61 * t3086;
    let t3089 = t839 * t3088;
    let t3090 = 22.07984838129906_f64 * t3089;
    let t3100 = t891 * t62 * t3086;
    let t3101 = t917 * t3100;
    let t3102 = 1.800081713982063_f64 * t3101;
    let t3103 = t127 * t567;
    let t3104 = t126 * t3103;
    (t3089, t3090, t3101, t3102, t3103, t3104)
}
