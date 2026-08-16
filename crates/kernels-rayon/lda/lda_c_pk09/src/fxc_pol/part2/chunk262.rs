//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 262/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk262(t1179: f64, t266: f64, t42: f64, t560: f64, t561: f64, t1168: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t1180 = t266 * t1179;
    let t1181 = 2.0970850588349075_f64 * t42;
    let t1182 = 7.140848730573871_f64 * t560;
    let t1183 = 8.043763671738963_f64 * t561;
    let t1184 = 3.0_f64 * t1168;
    let t1185 = t1181 - t1182 + t1183 - t1184;
    (t1180, t1181, t1182, t1183, t1184, t1185)
}
