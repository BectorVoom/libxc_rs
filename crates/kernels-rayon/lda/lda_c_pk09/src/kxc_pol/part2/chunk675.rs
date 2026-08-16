//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 675/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk675(t6300: f64, t6302: f64, t1468: f64, t467: f64, t1747: f64, t10: f64, t437: f64, t4977: f64, t440: f64) -> (f64, f64, f64) {
    let t6304 = 3.7610742193750633_f64 * t6300 * t6302;
    let t6305 = t467 * t1468;
    let t6306 = t6305 * t1747;
    let t6308 = 7.5221484387501265_f64 * t6306 * t6302;
    let t6318 = t4977 * t437 * t10;
    let t6319 = t6318 * t440;
    (t6304, t6308, t6319)
}
