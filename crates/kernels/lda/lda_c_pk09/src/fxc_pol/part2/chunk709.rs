//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 709/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk709<F: Float>(t1468: F, t506: F, t1747: F, t6302: F, t1931: F, t6488: F, t513: F, t15: F, t902: F, t505: F, t309: F, t6586: F) -> (F, F, F, F, F, F) {
    let t6925 = t506 * t1468;
    let t6926 = t6925 * t1747;
    let t6928 = F::cast_from(9.87466743489671_f64) * t6926 * t6302;
    let t6930 = F::cast_from(3.2915558116322368_f64) * t1931 * t6488;
    let t6932 = t513 * t513;
    let t6933 = F::new(1.0) / t6932;
    let t6938 = t15 * t902;
    let t6944 = t505 * t505;
    let t6945 = F::new(1.0) / t6944;
    let t6950 = t6586 * t309;
    (t6928, t6930, t6933, t6938, t6945, t6950)
}
