//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 612/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk612(t5039: f64, t1240: f64, t282: f64, t10: f64, t1267: f64) -> (f64, f64, f64) {
    let t5040 = 2.0_f64 * t5039;
    let t5041 = t1240 * t282;
    let t5042 = t5041 * t10;
    let t5043 = t5042 * t1267;
    (t5040, t5042, t5043)
}
