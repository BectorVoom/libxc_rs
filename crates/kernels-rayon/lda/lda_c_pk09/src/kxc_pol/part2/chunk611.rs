//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 611/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk611(t1318: f64, t5031: f64, t1287: f64, t1332: f64, t1435: f64, t10: f64, t289: f64, t4977: f64, t293: f64) -> (f64, f64, f64) {
    let t5032 = t1318 * t5031;
    let t5033 = t5032 * t1287;
    let t5035 = t1332 * t1435;
    let t5038 = t4977 * t289 * t10;
    let t5039 = t5038 * t293;
    (t5033, t5035, t5039)
}
