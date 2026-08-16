//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 709/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk709(t1468: f64, t506: f64, t1747: f64, t6302: f64, t1931: f64, t6488: f64, t513: f64, t15: f64, t902: f64, t505: f64, t309: f64, t6586: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t6925 = t506 * t1468;
    let t6926 = t6925 * t1747;
    let t6928 = 9.87466743489671_f64 * t6926 * t6302;
    let t6930 = 3.2915558116322368_f64 * t1931 * t6488;
    let t6932 = t513 * t513;
    let t6933 = 1.0_f64 / t6932;
    let t6938 = t15 * t902;
    let t6944 = t505 * t505;
    let t6945 = 1.0_f64 / t6944;
    let t6950 = t6586 * t309;
    (t6928, t6930, t6933, t6938, t6945, t6950)
}
