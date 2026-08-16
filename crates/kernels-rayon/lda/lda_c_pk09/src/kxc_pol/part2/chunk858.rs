//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 858/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk858(t8865: f64, t891: f64, t890: f64, t2143: f64, t650: f64, t844: f64, t164: f64, t61: f64, t7766: f64, t2149: f64, t849: f64, t7704: f64) -> (f64, f64, f64, f64, f64) {
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
